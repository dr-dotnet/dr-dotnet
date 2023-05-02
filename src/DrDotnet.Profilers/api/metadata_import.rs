use crate::{
    ffi::{
        mdMethodDef, CorMethodAttr, CorMethodImpl, MetaDataImport as FFIMetaDataImport, HRESULT,
        HRESULT::S_OK, WCHAR, CorTypeAttr,
    },
    MetadataImportTrait, MethodProps, TypeProps,
};
use std::{mem::MaybeUninit, ptr};
use widestring::U16CString;

use super::ffi::{mdTypeDef, mdGenericParam};

#[derive(Clone)]
pub struct MetadataImport {
    import: *const FFIMetaDataImport,
}

impl MetadataImport {
    pub fn new(metadata_import: *const FFIMetaDataImport) -> Self {
        MetadataImport {
            import: metadata_import,
        }
    }
    fn import(&self) -> &FFIMetaDataImport {
        unsafe { self.import.as_ref().unwrap() }
    }
}

impl MetadataImportTrait for MetadataImport {
    fn get_method_props(&self, mb: mdMethodDef) -> Result<MethodProps, HRESULT> {
        let mut name_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.import().GetMethodProps(
                mb,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                name_buffer_length.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        let mut class_token = MaybeUninit::uninit();
        let name_buffer_length = unsafe { name_buffer_length.assume_init() };
        let mut name_buffer = Vec::<WCHAR>::with_capacity(name_buffer_length as usize);
        unsafe { name_buffer.set_len(name_buffer_length as usize) };
        let mut name_length = MaybeUninit::uninit();
        let mut attr_flags = MaybeUninit::uninit();
        let mut sig = MaybeUninit::uninit();
        let mut sig_length = MaybeUninit::uninit();
        let mut rva = MaybeUninit::uninit();
        let mut impl_flags = MaybeUninit::uninit();
        let hr = unsafe {
            self.import().GetMethodProps(
                mb,
                class_token.as_mut_ptr(),
                name_buffer.as_mut_ptr(),
                name_buffer_length,
                name_length.as_mut_ptr(),
                attr_flags.as_mut_ptr(),
                sig.as_mut_ptr(),
                sig_length.as_mut_ptr(),
                rva.as_mut_ptr(),
                impl_flags.as_mut_ptr(),
            )
        };
        match hr {
            HRESULT::S_OK => {
                let class_token = unsafe { class_token.assume_init() };
                let name = U16CString::from_vec_with_nul(name_buffer)
                    .unwrap()
                    .to_string_lossy();
                let attr_flags = unsafe { attr_flags.assume_init() };
                let attr_flags = CorMethodAttr::from_bits(attr_flags).unwrap();
                let sig = unsafe { sig.assume_init() };
                let sig_length = unsafe { sig_length.assume_init() };
                let rva = unsafe { rva.assume_init() };
                let impl_flags = unsafe { impl_flags.assume_init() };
                let impl_flags = CorMethodImpl::from_bits(impl_flags).unwrap();
                Ok(MethodProps {
                    class_token,
                    name,
                    attr_flags,
                    sig,
                    sig_length,
                    rva,
                    impl_flags,
                })
            }
            _ => Err(hr),
        }
    }

    fn get_type_def_props(&self, td: crate::ffi::mdTypeDef) -> Result<crate::TypeProps, HRESULT> {
        let mut name_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.import().GetTypeDefProps(
                td,
                ptr::null_mut(),
                0,
                name_buffer_length.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut()
            )
        };

        let name_buffer_length = unsafe { name_buffer_length.assume_init() };
        let mut name_buffer = Vec::<WCHAR>::with_capacity(name_buffer_length as usize);
        unsafe { name_buffer.set_len(name_buffer_length as usize) };
        let mut pch_type_def = MaybeUninit::uninit();
        let mut type_def_flags = MaybeUninit::uninit();
        let mut base_type = MaybeUninit::uninit();
        let hr = unsafe {
            self.import().GetTypeDefProps(
                td,
                name_buffer.as_mut_ptr(),
                name_buffer_length,
                pch_type_def.as_mut_ptr(),
                type_def_flags.as_mut_ptr(),
                base_type.as_mut_ptr()
            )
        };

        match hr {
            HRESULT::S_OK => {
                let name = U16CString::from_vec_with_nul(name_buffer)
                    .unwrap()
                    .to_string_lossy();
                let type_def_flags = unsafe { type_def_flags.assume_init() };
                let type_def_flags = CorTypeAttr::from_bits(type_def_flags).unwrap();
                let base_type = unsafe { base_type.assume_init() };

                Ok(TypeProps {
                    name,
                    type_def_flags,
                    base_type
                })
            }
            _ => Err(hr),
        }
    }

    fn get_nested_class_props(&self, td: crate::ffi::mdTypeDef) -> Result<crate::ffi::mdTypeDef, HRESULT> {
        let mut enclosing_type_def = MaybeUninit::uninit();
        let hr = unsafe {
            self.import().GetNestedClassProps(
                td,
                enclosing_type_def.as_mut_ptr()
            )
        };

        match hr {
            HRESULT::S_OK => {
                let enclosing_type_def = unsafe { enclosing_type_def.assume_init() };
                Ok(enclosing_type_def)
            }
            _ => Err(hr),
        }
    }

    fn enum_generic_params(&self, td: crate::ffi::mdTypeDef) -> Result<Vec<crate::ffi::mdGenericParam>, HRESULT> {
        let mut enumerator_handle = MaybeUninit::uninit();
        let mut generic_param_buffer = Vec::<mdGenericParam>::with_capacity(10);
        unsafe { generic_param_buffer.set_len(10) };
        let mut params_fetched = MaybeUninit::uninit();

        info!("open 2");

        let hr = unsafe {
            self.import().EnumGenericParams(
                enumerator_handle.as_mut_ptr(),
                td,
                generic_param_buffer.as_mut_ptr(),
                1,
                params_fetched.as_mut_ptr()
            )
        };

        info!("mid");

        return Err(HRESULT::S_OK);

        // let result = match hr {
        //     HRESULT::S_OK => {
        //         let params_fetched = unsafe { params_fetched.assume_init() } as usize;
        //         info!("mid {}", params_fetched);
        //         generic_param_buffer.truncate(params_fetched);
        //         Ok(generic_param_buffer)
        //     }
        //     _ => Err(hr),
        // };

        // //info!("mid {:?}", result);

        // // Enumeration must be closed
        // unsafe {
        //     let enumerator_handle = enumerator_handle.assume_init();
        //     self.import().CloseEnum(enumerator_handle);
        // };

        // info!("close");

        // result
    }

    fn get_generic_params_props(&self, generic_param: crate::ffi::mdGenericParam) -> Result<crate::ffi::mdTypeDef, HRESULT> {
        info!("1");
        let mut pul_param_seq = MaybeUninit::uninit();
        let mut pdw_param_flags = MaybeUninit::uninit();
        let mut type_def = MaybeUninit::uninit();
        let mut reserved = MaybeUninit::uninit();
        let mut wz_name = MaybeUninit::uninit();
        let mut pch_name = MaybeUninit::uninit();
        let hr = unsafe {
            self.import().GetGenericParamProps(
                generic_param,
                pul_param_seq.as_mut_ptr(),
                pdw_param_flags.as_mut_ptr(),
                type_def.as_mut_ptr(),
                reserved.as_mut_ptr(),
                wz_name.as_mut_ptr(),
                0, // For now we don't care about the generic parameter name
                pch_name.as_mut_ptr()
            )
        };

        info!("2");

        match hr {
            HRESULT::S_OK => {
                let type_def = unsafe { type_def.assume_init() };
                Ok(type_def)
            }
            _ => Err(hr),
        }
    }

    fn get_version_string(&self) -> Result<String, HRESULT> {
        let mut buffer_length = MaybeUninit::uninit();
        unsafe {
            self.import().GetVersionString(
                ptr::null_mut(),
                0,
                buffer_length.as_mut_ptr()
            )
        };

        let buffer_length = unsafe { buffer_length.assume_init() };
        let mut name_buffer = Vec::<WCHAR>::with_capacity(buffer_length as usize);
        unsafe { name_buffer.set_len(buffer_length as usize) };
        let hr = unsafe {
            self.import().GetVersionString(
                name_buffer.as_mut_ptr(),
                buffer_length,
                ptr::null_mut()
            )
        };

        match hr {
            HRESULT::S_OK => {
                let version = U16CString::from_vec_with_nul(name_buffer)
                    .unwrap()
                    .to_string_lossy();

                Ok(version)
            }
            _ => Err(hr),
        }
    }
}
