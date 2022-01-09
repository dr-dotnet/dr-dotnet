use crate::{
    ffi::{
        mdMethodDef, CorMethodAttr, CorMethodImpl, MetaDataImport as FFIMetaDataImport, HRESULT,
        S_OK, WCHAR, CorTypeAttr,
    },
    MetadataImportTrait, MethodProps, TypeProps,
};
use std::{mem::MaybeUninit, ptr};
use widestring::U16CString;

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
            S_OK => {
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
            S_OK => {
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
}
