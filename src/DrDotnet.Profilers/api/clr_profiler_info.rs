use crate::*;
use crate::{
    ffi::{
        int, mdFieldDef, mdMethodDef, mdTypeDef, AppDomainID, AssemblyID, ClassID, ContextID, CorElementType, CorOpenFlags, CorProfilerFunctionEnum,
        CorProfilerInfo as FFICorProfilerInfo, CorProfilerModuleEnum, CorProfilerThreadEnum, FunctionEnter, FunctionEnter2, FunctionEnter3,
        FunctionEnter3WithInfo, FunctionID, FunctionIDMapper, FunctionIDMapper2, FunctionLeave, FunctionLeave2, FunctionLeave3, FunctionLeave3WithInfo,
        FunctionTailcall, FunctionTailcall2, FunctionTailcall3, FunctionTailcall3WithInfo, IMetaDataImport2, MethodMalloc, ModuleID, ObjectID,
        ObjectReferenceCallback, ReJITID, StackSnapshotCallback, ThreadID, BOOL, BYTE, COR_DEBUG_IL_TO_NATIVE_MAP, COR_FIELD_OFFSET, COR_IL_MAP,
        COR_PRF_CODE_INFO, COR_PRF_ELT_INFO, COR_PRF_EX_CLAUSE_INFO, COR_PRF_FRAME_INFO, COR_PRF_GC_GENERATION_RANGE, COR_PRF_HIGH_MONITOR,
        COR_PRF_MODULE_FLAGS, COR_PRF_MONITOR, COR_PRF_REJIT_FLAGS, COR_PRF_SNAPSHOT_INFO, COR_PRF_STATIC_TYPE, DWORD, GUID, HANDLE, HRESULT, LPCBYTE,
        UINT_PTR, ULONG, ULONG32, WCHAR,
    },
    utils::NameResolver,
    AppDomainInfo, ArrayClassInfo, ArrayObjectInfo, AssemblyInfo, ClassInfo, ClassInfo2, ClassLayout, CorProfilerInfo, CorProfilerInfo10, CorProfilerInfo2,
    CorProfilerInfo3, CorProfilerInfo4, CorProfilerInfo5, CorProfilerInfo6, CorProfilerInfo7, CorProfilerInfo8, CorProfilerInfo9, DynamicFunctionInfo,
    EnumNgenModuleMethodsInliningThisMethod, EventMask2, FunctionAndRejit, FunctionEnter3Info, FunctionInfo, FunctionInfo2, FunctionLeave3Info,
    FunctionTokenAndMetadata, IlFunctionBody, MetadataImport, ModuleInfo, ModuleInfo2, RuntimeInfo, StringLayout,
};
use itertools::Itertools;
use std::slice;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{mem::MaybeUninit, ptr};
use uuid::Uuid;
use widestring::U16CString;

#[derive(Clone)]
pub struct ClrProfilerInfo {
    info: *const FFICorProfilerInfo,
    attached_status: Arc<AtomicUsize>,
}

pub enum AttachedStatus {
    Attaching,
    Attached,
    Detaching,
    Detached,
}

impl Default for ClrProfilerInfo {
    fn default() -> Self {
        Self {
            info: core::ptr::null(),
            attached_status: Arc::new(AtomicUsize::new(AttachedStatus::Detached as usize)),
        }
    }
}

unsafe impl Send for ClrProfilerInfo {}

// Define some custom methods here, on top of those that are officially made available by the CLR profiling API
impl ClrProfilerInfo {
    pub fn new(cor_profiler_info: *const FFICorProfilerInfo) -> Self {
        ClrProfilerInfo {
            info: cor_profiler_info,
            attached_status: Arc::new(AtomicUsize::new(AttachedStatus::Detached as usize)),
        }
    }

    fn info(&self) -> &FFICorProfilerInfo {
        unsafe { self.info.as_ref().unwrap() }
    }

    pub fn get_attached_status(&self) -> AttachedStatus {
        match self.attached_status.load(Ordering::SeqCst) {
            x if x == AttachedStatus::Attaching as usize => AttachedStatus::Attaching,
            x if x == AttachedStatus::Attached as usize => AttachedStatus::Attached,
            x if x == AttachedStatus::Detaching as usize => AttachedStatus::Detaching,
            x if x == AttachedStatus::Detached as usize => AttachedStatus::Detached,
            _ => unreachable!(),
        }
    }

    pub(in crate::api) fn set_attached_status(&self, status: AttachedStatus) {
        self.attached_status.store(status as usize, Ordering::SeqCst);
    }

    pub fn get_gc_gen(generation_collected: &[ffi::BOOL]) -> i8 {
        let mut max_gen: i8 = -1;
        for gen in generation_collected {
            if *gen == 1 {
                max_gen += 1;
            }
        }
        return max_gen;
    }

    pub fn get_string_value(str_layout: &StringLayout, object_id: &ObjectID) -> String {
        let ptr = (*object_id + str_layout.buffer_offset as usize) as *const u16;
        let len = (*object_id + str_layout.string_length_offset as usize) as *const DWORD;
        // Could also be written as
        // let ptr = (*object_id as *const u8).offset(str_layout.buffer_offset as isize) as *const u16;
        // let len = (*object_id as *const u8).offset(str_layout.string_length_offset as isize) as *const DWORD;

        let slice = unsafe { slice::from_raw_parts(ptr, *len as usize) };
        String::from_utf16_lossy(slice).to_owned()

        // TODO: Benchmark widestring::U16CString::from_ptr_unchecked against String::from_utf16_lossy
        // unsafe {
        //     let str_len: u32 = *len_ptr;
        //     return widestring::U16CString::from_ptr_unchecked(str_ptr, str_len as usize).to_string_lossy()
        // };
    }

    // Return the name of type (with its namespace)
    fn get_type_name(&self, module_id: ModuleID, td: mdTypeDef) -> String {
        match self.get_module_metadata(module_id, CorOpenFlags::ofRead) {
            Ok(metadata) => match metadata.get_type_def_props(td) {
                Ok(type_props) => {
                    // If type is nested in another type, recursively get the name of the parent type to prefix it
                    let type_name = self.handle_nesting(type_props, &metadata, td, module_id);
                    type_name
                }
                Err(hresult) => {
                    warn!("metadata.get_type_def_props({}) failed ({:?})", td, hresult);
                    "unknown".to_owned()
                }
            },
            Err(hresult) => {
                warn!("info.get_module_metadata({}) failed ({:?})", module_id, hresult);
                "unknown".to_owned()
            }
        }
    }

    fn handle_nesting(&self, type_props: TypeProps, metadata: &MetadataImport, td: mdTypeDef, module_id: ModuleID) -> String {
        if type_props.type_def_flags.is_nested() {
            match metadata.get_nested_class_props(td) {
                Ok(nested_td) => {
                    let nesting_type_name = self.get_type_name(module_id, nested_td);
                    format!("{}.{}", nesting_type_name, type_props.name)
                }
                Err(hresult) => {
                    warn!("metadata.get_nested_class_props({}) failed ({:?})", td, hresult);
                    // Fallback to just using plain type name
                    type_props.name
                }
            }
        } else {
            // The type is not a nested type, simply return its name
            type_props.name
        }
    }
}

impl NameResolver for ClrProfilerInfo {
    // Returns a method name and the type where it is defined (namespaced) for a given FunctionID
    fn get_full_method_name(&self, method_id: FunctionID, class_id: ClassID) -> String {
        let class_name = if class_id == 0 {
            // Fallback to class name without generics if class_id is not provided
            match self.get_function_info(method_id) {
                Ok(function_info) => match self.get_token_and_metadata_from_function(method_id) {
                    Ok(f) => match f.metadata_import.get_method_props(f.token) {
                        Ok(method_props) => self.get_type_name(function_info.module_id, method_props.class_token),
                        Err(hresult) => {
                            warn!("metadata_import.get_method_props({}) failed ({:?})", f.token, hresult);
                            "unknown".to_owned()
                        }
                    },
                    Err(hresult) => {
                        warn!("info.get_token_and_metadata_from_function({}) failed ({:?})", method_id, hresult);
                        "unknown".to_owned()
                    }
                },
                Err(hresult) => {
                    warn!("info.get_function_info({}) failed ({:?})", method_id, hresult);
                    "unknown".to_owned()
                }
            }
        } else {
            // If class_id is known, we can try resolving the class name (including generics)
            self.get_class_name(class_id)
        };
        let method_name = match self.get_token_and_metadata_from_function(method_id) {
            Ok(f) => match f.metadata_import.get_method_props(f.token) {
                Ok(method_props) => method_props.name,
                Err(hresult) => {
                    warn!("metadata_import.get_method_props({}) failed ({:?})", f.token, hresult);
                    "unknown_method".to_owned()
                }
            },
            Err(hresult) => {
                warn!("info.get_token_and_metadata_from_function({}) failed ({:?})", method_id, hresult);
                "unknown_method".to_owned()
            }
        };
        format!("{}.{}", class_name, method_name)
    }

    // Returns a class name (namespaced) for a given ClassID
    fn get_class_name(&self, class_id: ClassID) -> String {
        // If the key doesn't exist, calculate the value
        let mut array_dimension = 0;
        let class_id = self.get_inner_type(class_id, &mut array_dimension);

        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-getclassidinfo-method
        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo2-getclassidinfo2-method
        let mut name = match self.get_class_id_info_2(class_id) {
            Ok(class_info) => {
                let name = self.get_type_name(class_info.module_id, class_info.token);
                let name = self.handle_generics(name, &class_info);
                name
            }
            Err(e) => format!("unknown_class ({:?})", e),
        };

        // Append array symbols []
        if array_dimension > 0 {
            name.reserve(array_dimension * 2);
            for _ in 0..array_dimension {
                name.push_str("[]");
            }
        }

        name
    }
}

impl ClrProfilerInfo {
    // If the type is an array, recursively drill until the base object type is found
    fn get_inner_type(&self, class_id: ClassID, array_dimension: &mut usize) -> ClassID {
        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-isarrayclass-method
        match self.is_array_class(class_id) {
            Ok(array_class_info) => {
                *array_dimension = *array_dimension + 1;
                // TODO: Handle array_class_info.rank
                if let Some(element_class_id) = array_class_info.element_class_id {
                    self.get_inner_type(element_class_id, array_dimension)
                } else {
                    error!("No element class id for array class object");
                    class_id
                }
            }
            Err(_) => class_id,
        }
    }

    fn handle_generics(&self, type_name: String, class_info: &ClassInfo2) -> String {
        let total_generic_types = class_info.type_args.len();

        if total_generic_types == 0 {
            return type_name;
        }

        // Before calling this function, generic types in a type name are hidden behind `N
        // where N is a number of generic parameters.
        // This method iterates over all occurrences of `N in a type name (there can be more than one,
        // for instance when dealing with nested types)
        // Whenever an occurrence is found, it replaces with by retreiving the N next generic
        // types from ClassInfo2::type_args.
        // Example:
        //  input: System.Collections.Generic.Dictionary`2.Entry
        // output: System.Collections.Generic.Dictionary<int, string>.Entry
        let mut start = 0;
        let mut current_generic_type_index = 0;
        let mut outstring = String::new();

        // Todo: Optimize this code
        while let Some(pos) = type_name[start..].find('`') {
            let absolute_pos = start + pos;

            outstring.push_str(&type_name[start..absolute_pos]);

            match type_name[absolute_pos + 1..absolute_pos + 2].parse::<usize>() {
                Ok(args_count) => {
                    // Recursively get the generic argument names
                    let arg_names = (0..args_count)
                        .into_iter()
                        .map(|i| {
                            let arg_class_id = class_info.type_args[current_generic_type_index];
                            let arg_name = self.get_class_name(arg_class_id);
                            current_generic_type_index += 1;
                            return arg_name;
                        })
                        .join(", ");

                    // Surrounds generic arguments with < >
                    outstring.push_str(&format!("<{}>", arg_names));
                }
                Err(_) => {
                    error!("We have an error...");
                }
            }

            // Change start to look for next group, if any
            start = absolute_pos + 2;
        }

        outstring.push_str(&type_name[start..]);

        outstring
    }
}

impl CorProfilerInfo for ClrProfilerInfo {
    fn get_class_from_object(&self, object_id: ObjectID) -> Result<ClassID, HRESULT> {
        let mut class_id = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetClassFromObject(object_id, class_id.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let class_id = unsafe { class_id.assume_init() };
                Ok(class_id)
            }
            _ => Err(hr),
        }
    }
    fn get_event_mask(&self) -> Result<COR_PRF_MONITOR, HRESULT> {
        let mut events = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetEventMask(events.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let events = unsafe { events.assume_init() };
                Ok(COR_PRF_MONITOR::from_bits(events).unwrap())
            }
            _ => Err(hr),
        }
    }
    fn get_function_from_ip(&self, ip: LPCBYTE) -> Result<FunctionID, HRESULT> {
        let mut function_id = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetFunctionFromIP(ip, function_id.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let function_id = unsafe { function_id.assume_init() };
                Ok(function_id)
            }
            _ => Err(hr),
        }
    }
    fn get_handle_from_thread(&self, thread_id: ThreadID) -> Result<HANDLE, HRESULT> {
        let mut handle = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetHandleFromThread(thread_id, handle.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let handle = unsafe { handle.assume_init() };
                Ok(handle)
            }
            _ => Err(hr),
        }
    }
    fn is_array_class(&self, class_id: ClassID) -> Result<ArrayClassInfo, HRESULT> {
        let mut element_type = MaybeUninit::uninit();
        let mut element_class_id = MaybeUninit::uninit();
        let mut rank = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .IsArrayClass(class_id, element_type.as_mut_ptr(), element_class_id.as_mut_ptr(), rank.as_mut_ptr())
        };
        match hr {
            HRESULT::S_OK => {
                let element_type = unsafe { element_type.assume_init() };
                let element_class_id = unsafe {
                    if !element_class_id.as_ptr().is_null() {
                        Some(element_class_id.assume_init())
                    } else {
                        None
                    }
                };
                let rank = unsafe { rank.assume_init() };
                Ok(ArrayClassInfo {
                    element_type: CorElementType::from(element_type),
                    element_class_id,
                    rank,
                })
            }
            _ => Err(hr),
        }
    }
    fn get_thread_info(&self, thread_id: ThreadID) -> Result<DWORD, HRESULT> {
        let mut win_32_thread_id = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetThreadInfo(thread_id, win_32_thread_id.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let win_32_thread_id = unsafe { win_32_thread_id.assume_init() };
                Ok(win_32_thread_id)
            }
            _ => Err(hr),
        }
    }
    fn get_current_thread_id(&self) -> Result<ThreadID, HRESULT> {
        let mut thread_id = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetCurrentThreadID(thread_id.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let thread_id = unsafe { thread_id.assume_init() };
                Ok(thread_id)
            }
            _ => Err(hr),
        }
    }
    fn get_class_id_info(&self, class_id: ClassID) -> Result<ClassInfo, HRESULT> {
        let mut module_id = MaybeUninit::uninit();
        let mut token = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetClassIDInfo(class_id, module_id.as_mut_ptr(), token.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let module_id = unsafe { module_id.assume_init() };
                let token = unsafe { token.assume_init() };
                Ok(ClassInfo { module_id, token })
            }
            _ => Err(hr),
        }
    }
    fn get_function_info(&self, function_id: FunctionID) -> Result<FunctionInfo, HRESULT> {
        let mut class_id = MaybeUninit::uninit();
        let mut module_id = MaybeUninit::uninit();
        let mut token = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetFunctionInfo(function_id, class_id.as_mut_ptr(), module_id.as_mut_ptr(), token.as_mut_ptr())
        };
        match hr {
            HRESULT::S_OK => {
                let class_id = unsafe { class_id.assume_init() };
                let module_id = unsafe { module_id.assume_init() };
                let token = unsafe { token.assume_init() };
                Ok(FunctionInfo { class_id, module_id, token })
            }
            _ => Err(hr),
        }
    }
    fn set_event_mask(&self, events: COR_PRF_MONITOR) -> Result<(), HRESULT> {
        let events = events.bits();
        let hr = unsafe { self.info().SetEventMask(events) };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn set_enter_leave_function_hooks(&self, func_enter: FunctionEnter, func_leave: FunctionLeave, func_tailcall: FunctionTailcall) -> Result<(), HRESULT> {
        let func_enter = func_enter as *const FunctionEnter;
        let func_leave = func_leave as *const FunctionLeave;
        let func_tailcall = func_tailcall as *const FunctionTailcall;
        let hr = unsafe { self.info().SetEnterLeaveFunctionHooks(func_enter, func_leave, func_tailcall) };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn set_function_id_mapper(&self, func: FunctionIDMapper) -> Result<(), HRESULT> {
        let func = func as *const FunctionIDMapper;
        let hr = unsafe { self.info().SetFunctionIDMapper(func) };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }

    fn get_token_and_metadata_from_function(&self, function_id: FunctionID) -> Result<FunctionTokenAndMetadata, HRESULT> {
        let mut metadata_import = MaybeUninit::uninit();
        let mut token = MaybeUninit::uninit();
        let riid = GUID::from(Uuid::parse_str("7DAC8207-D3AE-4C75-9B67-92801A497D44").unwrap()); // TODO: This needs to come from an IMetaDataImport implementation
        let hr = unsafe {
            self.info()
                .GetTokenAndMetaDataFromFunction(function_id, &riid, metadata_import.as_mut_ptr(), token.as_mut_ptr())
        };

        match hr {
            HRESULT::S_OK => {
                let metadata_import = unsafe { metadata_import.assume_init() };
                let token = unsafe { token.assume_init() };
                let metadata_import = super::MetadataImport::new(metadata_import);
                Ok(FunctionTokenAndMetadata { metadata_import, token })
            }
            _ => Err(hr),
        }
    }

    fn get_module_info(&self, module_id: ModuleID) -> Result<ModuleInfo, HRESULT> {
        let mut name_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info()
                .GetModuleInfo(module_id, ptr::null_mut(), 0, name_buffer_length.as_mut_ptr(), ptr::null_mut(), ptr::null_mut())
        };

        let mut base_load_address = MaybeUninit::uninit();
        let name_buffer_length = unsafe { name_buffer_length.assume_init() };
        let mut name_buffer = Vec::<WCHAR>::with_capacity(name_buffer_length as usize);
        unsafe { name_buffer.set_len(name_buffer_length as usize) };
        let mut name_length = MaybeUninit::uninit();
        let mut assembly_id = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetModuleInfo(
                module_id,
                base_load_address.as_mut_ptr(),
                name_buffer_length,
                name_length.as_mut_ptr(),
                name_buffer.as_mut_ptr(),
                assembly_id.as_mut_ptr(),
            )
        };
        match hr {
            HRESULT::S_OK => {
                let base_load_address = unsafe { base_load_address.assume_init() };
                let file_name = U16CString::from_vec_with_nul(name_buffer).unwrap().to_string_lossy();
                let assembly_id = unsafe { assembly_id.assume_init() };
                Ok(ModuleInfo {
                    base_load_address,
                    file_name,
                    assembly_id,
                })
            }
            _ => Err(hr),
        }
    }
    fn get_module_metadata(&self, module_id: ModuleID, open_flags: CorOpenFlags) -> Result<MetadataImport, HRESULT> {
        let mut metadata_import = MaybeUninit::uninit();
        let open_flags = open_flags.bits();
        let riid = IMetaDataImport2::IID;
        let hr = unsafe { self.info().GetModuleMetaData(module_id, open_flags, &riid, metadata_import.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let metadata_import = unsafe { metadata_import.assume_init().as_mut().unwrap() };
                let metadata_import = MetadataImport::new(metadata_import);
                Ok(metadata_import)
            }
            _ => Err(hr),
        }
    }
    fn get_il_function_body(&self, module_id: ModuleID, method_id: mdMethodDef) -> Result<IlFunctionBody, HRESULT> {
        let mut method_header = MaybeUninit::uninit();
        let mut method_size = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetILFunctionBody(module_id, method_id, method_header.as_mut_ptr(), method_size.as_mut_ptr())
        };

        match hr {
            HRESULT::S_OK => {
                let method_header = unsafe { method_header.assume_init() };
                let method_size = unsafe { method_size.assume_init() };
                Ok(IlFunctionBody { method_header, method_size })
            }
            _ => Err(hr),
        }
    }
    fn get_il_function_body_allocator(&self, module_id: ModuleID) -> Result<&mut MethodMalloc, HRESULT> {
        let mut malloc = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetILFunctionBodyAllocator(module_id, malloc.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let malloc = unsafe { malloc.assume_init().as_mut().unwrap() };
                Ok(malloc)
            }
            _ => Err(hr),
        }
    }
    fn set_il_function_body(&self, module_id: ModuleID, method_id: mdMethodDef, new_il_method_header: LPCBYTE) -> Result<(), HRESULT> {
        let hr = unsafe { self.info().SetILFunctionBody(module_id, method_id, new_il_method_header) };

        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn get_app_domain_info(&self, app_domain_id: AppDomainID) -> Result<AppDomainInfo, HRESULT> {
        // get app domain name length, with zero-length buffer call
        let mut name_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info()
                .GetAppDomainInfo(app_domain_id, 0, name_buffer_length.as_mut_ptr(), ptr::null_mut(), ptr::null_mut())
        };

        let name_buffer_length = unsafe { name_buffer_length.assume_init() };
        let mut name_buffer = Vec::<WCHAR>::with_capacity(name_buffer_length as usize);
        unsafe { name_buffer.set_len(name_buffer_length as usize) };
        let mut name_length = MaybeUninit::uninit();
        let mut process_id = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetAppDomainInfo(
                app_domain_id,
                name_buffer_length,
                name_length.as_mut_ptr(),
                name_buffer.as_mut_ptr(),
                process_id.as_mut_ptr(),
            )
        };
        match hr {
            HRESULT::S_OK => {
                let name = U16CString::from_vec_with_nul(name_buffer).unwrap().to_string_lossy();
                let process_id = unsafe { process_id.assume_init() };
                Ok(AppDomainInfo { name, process_id })
            }
            _ => Err(hr),
        }
    }
    fn get_assembly_info(&self, assembly_id: AssemblyID) -> Result<AssemblyInfo, HRESULT> {
        // get assembly name length, with zero-length buffer call
        let mut name_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info().GetAssemblyInfo(
                assembly_id,
                0,
                name_buffer_length.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        let name_buffer_length = unsafe { name_buffer_length.assume_init() };
        let mut name_buffer = Vec::<WCHAR>::with_capacity(name_buffer_length as usize);
        unsafe { name_buffer.set_len(name_buffer_length as usize) };
        let mut name_length = MaybeUninit::uninit();
        let mut app_domain_id = MaybeUninit::uninit();
        let mut module_id = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetAssemblyInfo(
                assembly_id,
                name_buffer_length,
                name_length.as_mut_ptr(),
                name_buffer.as_mut_ptr(),
                app_domain_id.as_mut_ptr(),
                module_id.as_mut_ptr(),
            )
        };
        match hr {
            HRESULT::S_OK => {
                let name = U16CString::from_vec_with_nul(name_buffer).unwrap().to_string_lossy();
                let app_domain_id = unsafe { app_domain_id.assume_init() };
                let module_id = unsafe { module_id.assume_init() };
                Ok(AssemblyInfo {
                    name,
                    app_domain_id,
                    module_id,
                })
            }
            _ => Err(hr),
        }
    }
    fn force_gc(&self) -> Result<(), HRESULT> {
        let hr = unsafe { self.info().ForceGC() };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn set_il_instrumented_code_map(&self, function_id: FunctionID, start_jit: bool, il_map_entries: &[COR_IL_MAP]) -> Result<(), HRESULT> {
        let start_jit: BOOL = if start_jit { 1 } else { 0 };
        let hr = unsafe {
            self.info()
                .SetILInstrumentedCodeMap(function_id, start_jit, il_map_entries.len() as ULONG, il_map_entries.as_ptr())
        };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn get_thread_context(&self, thread_id: ThreadID) -> Result<ContextID, HRESULT> {
        let mut context_id = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetThreadContext(thread_id, context_id.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let context_id = unsafe { context_id.assume_init() };
                Ok(context_id)
            }
            _ => Err(hr),
        }
    }
    fn get_il_to_native_mapping(&self, function_id: FunctionID) -> Result<Vec<COR_DEBUG_IL_TO_NATIVE_MAP>, HRESULT> {
        let mut map_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info()
                .GetILToNativeMapping(function_id, 0, map_buffer_length.as_mut_ptr(), ptr::null_mut())
        };

        let map_buffer_length = unsafe { map_buffer_length.assume_init() };
        let mut map = Vec::<COR_DEBUG_IL_TO_NATIVE_MAP>::with_capacity(map_buffer_length as usize);
        unsafe { map.set_len(map_buffer_length as usize) };
        let mut map_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetILToNativeMapping(function_id, map_buffer_length, map_length.as_mut_ptr(), map.as_mut_ptr())
        };
        match hr {
            HRESULT::S_OK => Ok(map),
            _ => Err(hr),
        }
    }
}

impl CorProfilerInfo2 for ClrProfilerInfo {
    fn do_stack_snapshot(
        &self,
        thread: ThreadID,
        callback: StackSnapshotCallback,
        info_flags: COR_PRF_SNAPSHOT_INFO,
        client_data: *const std::ffi::c_void, // TODO: How will ownership of this client_data work? Needs to leak, what about cleanup?
        context: *const BYTE, // TODO: This should be a Win32 CONTEXT structure. This is CPU-arch dependent though, how to implement? What about ownership?
        context_size: u32,
    ) -> Result<(), HRESULT> {
        let callback = callback as *const StackSnapshotCallback;
        let hr = unsafe {
            self.info()
                .DoStackSnapshot(thread, callback, info_flags as ULONG32, client_data, context, context_size)
        };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }

    fn set_enter_leave_function_hooks_2(
        &self,
        func_enter: FunctionEnter2,
        func_leave: FunctionLeave2,
        func_tailcall: FunctionTailcall2,
    ) -> Result<(), HRESULT> {
        let func_enter = func_enter as *const FunctionEnter2;
        let func_leave = func_leave as *const FunctionLeave2;
        let func_tailcall = func_tailcall as *const FunctionTailcall2;
        let hr = unsafe { self.info().SetEnterLeaveFunctionHooks2(func_enter, func_leave, func_tailcall) };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }

    fn get_function_info_2(&self, func_id: FunctionID, frame_info: COR_PRF_FRAME_INFO) -> Result<FunctionInfo2, HRESULT> {
        // get type args length, with zero-length buffer call
        let mut type_args_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info().GetFunctionInfo2(
                func_id,
                frame_info,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                type_args_buffer_length.as_mut_ptr(),
                ptr::null_mut(),
            )
        };

        let type_args_buffer_length = unsafe { type_args_buffer_length.assume_init() };
        let mut type_args = Vec::<ClassID>::with_capacity(type_args_buffer_length as usize);
        unsafe { type_args.set_len(type_args_buffer_length as usize) };

        let mut class_id = MaybeUninit::uninit();
        let mut module_id = MaybeUninit::uninit();
        let mut token = MaybeUninit::uninit();
        let mut type_args_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetFunctionInfo2(
                func_id,
                frame_info,
                class_id.as_mut_ptr(),
                module_id.as_mut_ptr(),
                token.as_mut_ptr(),
                type_args_buffer_length,
                type_args_length.as_mut_ptr(),
                type_args.as_mut_ptr(),
            )
        };

        match hr {
            HRESULT::S_OK => {
                let class_id = unsafe { class_id.assume_init() };
                let module_id = unsafe { module_id.assume_init() };
                let token = unsafe { token.assume_init() };
                Ok(FunctionInfo2 {
                    class_id,
                    module_id,
                    token,
                    type_args,
                })
            }
            _ => Err(hr),
        }
    }

    fn get_class_layout(&self, class_id: ClassID) -> Result<ClassLayout, HRESULT> {
        // get field offset length, with zero-length buffer call
        let mut field_offset_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info()
                .GetClassLayout(class_id, ptr::null_mut(), 0, field_offset_buffer_length.as_mut_ptr(), ptr::null_mut())
        };

        let field_offset_buffer_length = unsafe { field_offset_buffer_length.assume_init() };
        let mut field_offset = Vec::<COR_FIELD_OFFSET>::with_capacity(field_offset_buffer_length as usize);
        unsafe { field_offset.set_len(field_offset_buffer_length as usize) };

        let mut field_offset_length = MaybeUninit::uninit();
        let mut class_size_bytes = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetClassLayout(
                class_id,
                field_offset.as_mut_ptr(),
                field_offset_buffer_length,
                field_offset_length.as_mut_ptr(),
                class_size_bytes.as_mut_ptr(),
            )
        };

        match hr {
            HRESULT::S_OK => {
                let class_size_bytes = unsafe { class_size_bytes.assume_init() };
                Ok(ClassLayout {
                    field_offset,
                    class_size_bytes,
                })
            }
            _ => Err(hr),
        }
    }

    fn get_class_id_info_2(&self, class_id: ClassID) -> Result<ClassInfo2, HRESULT> {
        // get type args length, with zero-length buffer call
        let mut type_args_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info().GetClassIDInfo2(
                class_id,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                type_args_buffer_length.as_mut_ptr(),
                ptr::null_mut(),
            )
        };

        let type_args_buffer_length = unsafe { type_args_buffer_length.assume_init() };
        let mut type_args = Vec::<ClassID>::with_capacity(type_args_buffer_length as usize);
        unsafe { type_args.set_len(type_args_buffer_length as usize) };

        let mut module_id = MaybeUninit::uninit();
        let mut token = MaybeUninit::uninit();
        let mut parent_class_id = MaybeUninit::uninit();
        let mut type_args_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetClassIDInfo2(
                class_id,
                module_id.as_mut_ptr(),
                token.as_mut_ptr(),
                parent_class_id.as_mut_ptr(),
                type_args_buffer_length,
                type_args_length.as_mut_ptr(),
                type_args.as_mut_ptr(),
            )
        };

        match hr {
            HRESULT::S_OK => {
                let module_id = unsafe { module_id.assume_init() };
                let token = unsafe { token.assume_init() };
                let parent_class_id = unsafe { parent_class_id.assume_init() };
                Ok(ClassInfo2 {
                    module_id,
                    token,
                    parent_class_id,
                    type_args,
                })
            }
            _ => {
                warn!("info.get_class_id_info_2({}) failed ({:?})", class_id, hr);
                Err(hr)
            }
        }
    }

    fn get_code_info_2(&self, function_id: FunctionID) -> Result<Vec<COR_PRF_CODE_INFO>, HRESULT> {
        let mut code_info_buffer_length = MaybeUninit::uninit();
        unsafe { self.info().GetCodeInfo2(function_id, 0, code_info_buffer_length.as_mut_ptr(), ptr::null_mut()) };

        let code_info_buffer_length = unsafe { code_info_buffer_length.assume_init() };
        let mut code_info = Vec::<COR_PRF_CODE_INFO>::with_capacity(code_info_buffer_length as usize);
        unsafe { code_info.set_len(code_info_buffer_length as usize) };

        let mut code_info_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetCodeInfo2(function_id, code_info_buffer_length, code_info_length.as_mut_ptr(), code_info.as_mut_ptr())
        };

        match hr {
            HRESULT::S_OK => Ok(code_info),
            _ => Err(hr),
        }
    }

    fn get_class_from_token_and_type_args(&self, module_id: ModuleID, type_def: mdTypeDef, type_args: Option<&[ClassID]>) -> Result<ClassID, HRESULT> {
        let mut class_id = 0 as ClassID;
        // let (type_args, type_args_length) = match type_args {
        //     Some(args) => (args.as_ptr(), args.len()),
        //     None => (ptr::null(), 0),
        // };
        let hr = unsafe { self.info().GetClassFromTokenAndTypeArgs(module_id, type_def, 0, ptr::null(), &mut class_id) };

        match hr {
            HRESULT::S_OK => {
                let class_id = class_id;
                Ok(class_id)
            }
            _ => {
                error!("class_id: {}, module_id: {}, type_def: {}", class_id, module_id, type_def);
                Err(hr)
            }
        }
    }

    fn get_function_from_token_and_type_args(
        &self,
        module_id: ModuleID,
        func_def: mdMethodDef,
        class_id: ClassID,
        type_args: Option<&[ClassID]>,
    ) -> Result<FunctionID, HRESULT> {
        let mut function_id = MaybeUninit::uninit();
        let (type_args, type_args_length) = match type_args {
            Some(args) => (args.as_ptr(), args.len()),
            None => (ptr::null(), 0),
        };
        let hr = unsafe {
            self.info()
                .GetFunctionFromTokenAndTypeArgs(module_id, func_def, class_id, type_args_length as ULONG32, type_args, function_id.as_mut_ptr())
        };

        match hr {
            HRESULT::S_OK => {
                let function_id = unsafe { function_id.assume_init() };
                Ok(function_id)
            }
            _ => Err(hr),
        }
    }

    fn get_array_object_info(&self, object_id: ObjectID, dimensions: u32) -> Result<ArrayObjectInfo, HRESULT> {
        let mut dimension_sizes = Vec::<ULONG32>::with_capacity(dimensions as usize);
        unsafe { dimension_sizes.set_len(dimensions as usize) };
        let mut dimension_lower_bounds = Vec::<int>::with_capacity(dimensions as usize);
        unsafe { dimension_lower_bounds.set_len(dimensions as usize) };

        let mut data = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetArrayObjectInfo(
                object_id,
                dimensions,
                dimension_sizes.as_mut_ptr(),
                dimension_lower_bounds.as_mut_ptr(),
                data.as_mut_ptr(),
            )
        };

        match hr {
            HRESULT::S_OK => {
                let data = unsafe { data.assume_init() };
                Ok(ArrayObjectInfo {
                    dimension_sizes,
                    dimension_lower_bounds,
                    data,
                })
            }
            _ => Err(hr),
        }
    }

    fn get_box_class_layout(&self, class_id: ClassID) -> Result<u32, HRESULT> {
        let mut buffer_offset = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetBoxClassLayout(class_id, buffer_offset.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let buffer_offset = unsafe { buffer_offset.assume_init() };
                Ok(buffer_offset)
            }
            _ => Err(hr),
        }
    }

    fn get_thread_app_domain(&self, thread_id: ThreadID) -> Result<AppDomainID, HRESULT> {
        let mut app_domain_id = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetThreadAppDomain(thread_id, app_domain_id.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let app_domain_id = unsafe { app_domain_id.assume_init() };
                Ok(app_domain_id)
            }
            _ => Err(hr),
        }
    }

    fn get_rva_static_address(&self, class_id: ClassID, field_token: mdFieldDef) -> Result<*const std::ffi::c_void, HRESULT> {
        let mut address = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetRVAStaticAddress(class_id, field_token, address.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let address = unsafe { address.assume_init() };
                Ok(address)
            }
            _ => Err(hr),
        }
    }

    fn get_app_domain_static_address(
        &self,
        class_id: ClassID,
        field_token: mdFieldDef,
        app_domain_id: AppDomainID,
    ) -> Result<*const std::ffi::c_void, HRESULT> {
        let mut address = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetAppDomainStaticAddress(class_id, field_token, app_domain_id, address.as_mut_ptr())
        };

        match hr {
            HRESULT::S_OK => {
                let address = unsafe { address.assume_init() };
                Ok(address)
            }
            _ => Err(hr),
        }
    }

    fn get_thread_static_address(&self, class_id: ClassID, field_token: mdFieldDef, thread_id: ThreadID) -> Result<*const std::ffi::c_void, HRESULT> {
        let mut address = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetThreadStaticAddress(class_id, field_token, thread_id, address.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let address = unsafe { address.assume_init() };
                Ok(address)
            }
            _ => Err(hr),
        }
    }

    fn get_context_static_address(&self, class_id: ClassID, field_token: mdFieldDef, context_id: ContextID) -> Result<*const std::ffi::c_void, HRESULT> {
        let mut address = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetContextStaticAddress(class_id, field_token, context_id, address.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let address = unsafe { address.assume_init() };
                Ok(address)
            }
            _ => Err(hr),
        }
    }

    fn get_static_field_info(&self, class_id: ClassID, field_token: mdFieldDef) -> Result<COR_PRF_STATIC_TYPE, HRESULT> {
        let mut field_info = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetStaticFieldInfo(class_id, field_token, field_info.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let field_info = unsafe { field_info.assume_init() };
                Ok(field_info)
            }
            _ => Err(hr),
        }
    }
    fn get_generation_bounds(&self) -> Result<Vec<COR_PRF_GC_GENERATION_RANGE>, HRESULT> {
        let mut ranges_buffer_length = MaybeUninit::uninit();
        unsafe { self.info().GetGenerationBounds(0, ranges_buffer_length.as_mut_ptr(), ptr::null_mut()) };

        let ranges_buffer_length = unsafe { ranges_buffer_length.assume_init() };
        let mut ranges = Vec::<COR_PRF_GC_GENERATION_RANGE>::with_capacity(ranges_buffer_length as usize);
        unsafe { ranges.set_len(ranges_buffer_length as usize) };

        let mut ranges_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetGenerationBounds(ranges_buffer_length, ranges_length.as_mut_ptr(), ranges.as_mut_ptr())
        };

        match hr {
            HRESULT::S_OK => Ok(ranges),
            _ => Err(hr),
        }
    }
    fn get_object_generation(&self, object_id: ObjectID) -> Result<COR_PRF_GC_GENERATION_RANGE, HRESULT> {
        let mut range = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetObjectGeneration(object_id, range.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let range = unsafe { range.assume_init() };
                Ok(range)
            }
            _ => Err(hr),
        }
    }
    fn get_notified_exception_clause_info(&self) -> Result<COR_PRF_EX_CLAUSE_INFO, HRESULT> {
        let mut exception_clause_info = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetNotifiedExceptionClauseInfo(exception_clause_info.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let exception_clause_info = unsafe { exception_clause_info.assume_init() };
                Ok(exception_clause_info)
            }
            _ => Err(hr),
        }
    }
}

impl CorProfilerInfo3 for ClrProfilerInfo {
    fn enum_jited_functions(&self) -> Result<&mut CorProfilerFunctionEnum, HRESULT> {
        let mut function_enum = MaybeUninit::uninit();
        let hr = unsafe { self.info().EnumJITedFunctions(function_enum.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let function_enum = unsafe { function_enum.assume_init().as_mut().unwrap() };
                Ok(function_enum)
            }
            _ => Err(hr),
        }
    }
    fn request_profiler_detach(&self, expected_completion_milliseconds: u32) -> Result<(), HRESULT> {
        match self.attached_status.compare_exchange(
            AttachedStatus::Attached as usize,
            AttachedStatus::Detaching as usize,
            Ordering::Acquire,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                let hr = unsafe { self.info().RequestProfilerDetach(expected_completion_milliseconds) };

                match hr {
                    HRESULT::S_OK => Ok(()),
                    _ => Err(hr),
                }
            }
            Err(current) => match current {
                x if x == AttachedStatus::Detaching as usize => Err(HRESULT::CORPROF_E_PROFILER_DETACHING),
                _ => Err(HRESULT::CORPROF_E_UNSUPPORTED_CALL_SEQUENCE),
            },
        }
    }
    fn set_function_id_mapper_2(&self, func: FunctionIDMapper2, client_data: *const std::ffi::c_void) -> Result<(), HRESULT> {
        let func = func as *const FunctionIDMapper2;
        let hr = unsafe { self.info().SetFunctionIDMapper2(func, client_data) };

        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn get_string_layout_2(&self) -> Result<StringLayout, HRESULT> {
        let mut string_length_offset = MaybeUninit::uninit();
        let mut buffer_offset = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetStringLayout2(string_length_offset.as_mut_ptr(), buffer_offset.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let string_length_offset = unsafe { string_length_offset.assume_init() };
                let buffer_offset = unsafe { buffer_offset.assume_init() };
                Ok(StringLayout {
                    string_length_offset,
                    buffer_offset,
                })
            }
            _ => Err(hr),
        }
    }
    fn set_enter_leave_function_hooks_3(
        &self,
        func_enter_3: FunctionEnter3,
        func_leave_3: FunctionLeave3,
        func_tailcall_3: FunctionTailcall3,
    ) -> Result<(), HRESULT> {
        let func_enter_3 = func_enter_3 as *const FunctionEnter3;
        let func_leave_3 = func_leave_3 as *const FunctionLeave3;
        let func_tailcall_3 = func_tailcall_3 as *const FunctionTailcall3;
        let hr = unsafe { self.info().SetEnterLeaveFunctionHooks3(func_enter_3, func_leave_3, func_tailcall_3) };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn set_enter_leave_function_hooks_3_with_info(
        &self,
        func_enter_3_with_info: FunctionEnter3WithInfo,
        func_leave_3_with_info: FunctionLeave3WithInfo,
        func_tailcall_3_with_info: FunctionTailcall3WithInfo,
    ) -> Result<(), HRESULT> {
        let func_enter_3_with_info = func_enter_3_with_info as *const FunctionEnter3WithInfo;
        let func_leave_3_with_info = func_leave_3_with_info as *const FunctionLeave3WithInfo;
        let func_tailcall_3_with_info = func_tailcall_3_with_info as *const FunctionTailcall3WithInfo;
        let hr = unsafe {
            self.info()
                .SetEnterLeaveFunctionHooks3WithInfo(func_enter_3_with_info, func_leave_3_with_info, func_tailcall_3_with_info)
        };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn get_function_enter_3_info(&self, function_id: FunctionID, elt_info: COR_PRF_ELT_INFO) -> Result<FunctionEnter3Info, HRESULT> {
        let mut frame_info = MaybeUninit::uninit();
        let mut argument_info_length = MaybeUninit::uninit();
        let mut argument_info = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetFunctionEnter3Info(
                function_id,
                elt_info,
                frame_info.as_mut_ptr(),
                argument_info_length.as_mut_ptr(),
                argument_info.as_mut_ptr(),
            )
        };

        match hr {
            HRESULT::S_OK => {
                let frame_info = unsafe { frame_info.assume_init() };
                let argument_info_length = unsafe { argument_info_length.assume_init() };
                // TODO: Is there any tricky stuff we need to do to allocate the correct size for the argument_info struct?
                let argument_info = unsafe { argument_info.assume_init() };
                Ok(FunctionEnter3Info {
                    frame_info,
                    argument_info_length,
                    argument_info,
                })
            }
            _ => Err(hr),
        }
    }
    fn get_function_leave_3_info(&self, function_id: FunctionID, elt_info: COR_PRF_ELT_INFO) -> Result<FunctionLeave3Info, HRESULT> {
        let mut frame_info = MaybeUninit::uninit();
        let mut retval_range = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetFunctionLeave3Info(function_id, elt_info, frame_info.as_mut_ptr(), retval_range.as_mut_ptr())
        };

        match hr {
            HRESULT::S_OK => {
                let frame_info = unsafe { frame_info.assume_init() };
                let retval_range = unsafe { retval_range.assume_init() };
                Ok(FunctionLeave3Info { frame_info, retval_range })
            }
            _ => Err(hr),
        }
    }
    fn get_function_tailcall_3_info(&self, function_id: FunctionID, elt_info: COR_PRF_ELT_INFO) -> Result<COR_PRF_FRAME_INFO, HRESULT> {
        let mut frame_info = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetFunctionTailcall3Info(function_id, elt_info, frame_info.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let frame_info = unsafe { frame_info.assume_init() };
                Ok(frame_info)
            }
            _ => Err(hr),
        }
    }
    fn enum_modules(&self) -> Result<&mut CorProfilerModuleEnum, HRESULT> {
        let mut module_enum = MaybeUninit::uninit();
        let hr = unsafe { self.info().EnumModules(module_enum.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let module_enum = unsafe { module_enum.assume_init().as_mut().unwrap() };
                Ok(module_enum)
            }
            _ => Err(hr),
        }
    }
    fn get_runtime_information(&self) -> Result<RuntimeInfo, HRESULT> {
        let mut version_string_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info().GetRuntimeInformation(
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                version_string_buffer_length.as_mut_ptr(),
                ptr::null_mut(),
            )
        };

        let version_string_buffer_length = unsafe { version_string_buffer_length.assume_init() };
        let mut version_string_buffer = Vec::<WCHAR>::with_capacity(version_string_buffer_length as usize);
        unsafe { version_string_buffer.set_len(version_string_buffer_length as usize) };

        let mut clr_instance_id = MaybeUninit::uninit();
        let mut runtime_type = MaybeUninit::uninit();
        let mut major_version = MaybeUninit::uninit();
        let mut minor_version = MaybeUninit::uninit();
        let mut build_number = MaybeUninit::uninit();
        let mut qfe_version = MaybeUninit::uninit();
        let mut version_string_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetRuntimeInformation(
                clr_instance_id.as_mut_ptr(),
                runtime_type.as_mut_ptr(),
                major_version.as_mut_ptr(),
                minor_version.as_mut_ptr(),
                build_number.as_mut_ptr(),
                qfe_version.as_mut_ptr(),
                version_string_buffer_length,
                version_string_length.as_mut_ptr(),
                version_string_buffer.as_mut_ptr(),
            )
        };

        match hr {
            HRESULT::S_OK => {
                let clr_instance_id = unsafe { clr_instance_id.assume_init() };
                let runtime_type = unsafe { runtime_type.assume_init() };
                let major_version = unsafe { major_version.assume_init() };
                let minor_version = unsafe { minor_version.assume_init() };
                let build_number = unsafe { build_number.assume_init() };
                let qfe_version = unsafe { qfe_version.assume_init() };
                let version_string = U16CString::from_vec_with_nul(version_string_buffer).unwrap().to_string_lossy();
                Ok(RuntimeInfo {
                    clr_instance_id,
                    runtime_type,
                    major_version,
                    minor_version,
                    build_number,
                    qfe_version,
                    version_string,
                })
            }
            _ => Err(hr),
        }
    }
    fn get_thread_static_address_2(
        &self,
        class_id: ClassID,
        field_token: mdFieldDef,
        app_domain_id: AppDomainID,
        thread_id: ThreadID,
    ) -> Result<*const std::ffi::c_void, HRESULT> {
        let mut address = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetThreadStaticAddress2(class_id, field_token, app_domain_id, thread_id, address.as_mut_ptr())
        };

        match hr {
            HRESULT::S_OK => {
                let address = unsafe { address.assume_init() };
                Ok(address)
            }
            _ => Err(hr),
        }
    }
    fn get_app_domains_containing_module(&self, module_id: ModuleID) -> Result<Vec<AppDomainID>, HRESULT> {
        let mut app_domains_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info()
                .GetAppDomainsContainingModule(module_id, 0, app_domains_buffer_length.as_mut_ptr(), ptr::null_mut())
        };

        let app_domains_buffer_length = unsafe { app_domains_buffer_length.assume_init() };
        let mut app_domains_buffer = Vec::<AppDomainID>::with_capacity(app_domains_buffer_length as usize);
        unsafe { app_domains_buffer.set_len(app_domains_buffer_length as usize) };

        let mut app_domains_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetAppDomainsContainingModule(
                module_id,
                app_domains_buffer_length,
                app_domains_length.as_mut_ptr(),
                app_domains_buffer.as_mut_ptr(),
            )
        };

        match hr {
            HRESULT::S_OK => Ok(app_domains_buffer),
            _ => Err(hr),
        }
    }
    fn get_module_info_2(&self, module_id: ModuleID) -> Result<ModuleInfo2, HRESULT> {
        let mut file_name_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info().GetModuleInfo2(
                module_id,
                ptr::null_mut(),
                0,
                file_name_buffer_length.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        let file_name_buffer_length = unsafe { file_name_buffer_length.assume_init() };
        let mut file_name_buffer = Vec::<WCHAR>::with_capacity(file_name_buffer_length as usize);
        unsafe { file_name_buffer.set_len(file_name_buffer_length as usize) };

        let mut base_load_address = MaybeUninit::uninit();
        let mut file_name_length = MaybeUninit::uninit();
        let mut assembly_id = MaybeUninit::uninit();
        let mut module_flags = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetModuleInfo2(
                module_id,
                base_load_address.as_mut_ptr(),
                file_name_buffer_length,
                file_name_length.as_mut_ptr(),
                file_name_buffer.as_mut_ptr(),
                assembly_id.as_mut_ptr(),
                module_flags.as_mut_ptr(),
            )
        };

        match hr {
            HRESULT::S_OK => {
                let base_load_address = unsafe { base_load_address.assume_init() };
                let assembly_id = unsafe { assembly_id.assume_init() };
                let module_flags = unsafe { module_flags.assume_init() };
                let module_flags = COR_PRF_MODULE_FLAGS::from_bits(module_flags).unwrap();
                let file_name = U16CString::from_vec_with_nul(file_name_buffer).unwrap().to_string_lossy();
                Ok(ModuleInfo2 {
                    base_load_address,
                    file_name,
                    assembly_id,
                    module_flags,
                })
            }
            _ => Err(hr),
        }
    }
}
impl CorProfilerInfo4 for ClrProfilerInfo {
    fn enum_threads(&self) -> Result<&mut CorProfilerThreadEnum, HRESULT> {
        let mut thread_enum = MaybeUninit::uninit();
        let hr = unsafe { self.info().EnumThreads(thread_enum.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let thread_enum = unsafe { thread_enum.assume_init().as_mut().unwrap() };
                Ok(thread_enum)
            }
            _ => Err(hr),
        }
    }
    fn initialize_current_thread(&self) -> Result<(), HRESULT> {
        let hr = unsafe { self.info().InitializeCurrentThread() };

        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn request_rejit(
        &self,
        module_ids: &[ModuleID],
        method_ids: &[mdMethodDef], // TODO: Maybe make the pairs actual tuples? Simple zip op.
    ) -> Result<(), HRESULT> {
        let methods_length = module_ids.len() as u32;
        let module_ids = module_ids.as_ptr();
        let method_ids = method_ids.as_ptr();
        let hr = unsafe { self.info().RequestReJIT(methods_length, module_ids, method_ids) };

        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn request_revert(
        &self,
        module_ids: &[ModuleID],
        method_ids: &[mdMethodDef], // TODO: Maybe make the pairs actual tuples? Simple zip op.
    ) -> Result<Vec<HRESULT>, HRESULT> {
        let methods_length = module_ids.len() as u32;
        let module_ids = module_ids.as_ptr();
        let method_ids = method_ids.as_ptr();
        let mut statuses_buffer = Vec::<HRESULT>::with_capacity(methods_length as usize);
        unsafe { statuses_buffer.set_len(methods_length as usize) };
        let hr = unsafe { self.info().RequestRevert(methods_length, module_ids, method_ids, statuses_buffer.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => Ok(statuses_buffer),
            _ => Err(hr),
        }
    }
    fn get_code_info_3(&self, function_id: FunctionID, rejit_id: ReJITID) -> Result<Vec<COR_PRF_CODE_INFO>, HRESULT> {
        let mut code_info_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info()
                .GetCodeInfo3(function_id, rejit_id, 0, code_info_buffer_length.as_mut_ptr(), ptr::null_mut())
        };

        let code_info_buffer_length = unsafe { code_info_buffer_length.assume_init() };
        let mut code_info = Vec::<COR_PRF_CODE_INFO>::with_capacity(code_info_buffer_length as usize);
        unsafe { code_info.set_len(code_info_buffer_length as usize) };

        let mut code_info_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetCodeInfo3(
                function_id,
                rejit_id,
                code_info_buffer_length,
                code_info_length.as_mut_ptr(),
                code_info.as_mut_ptr(),
            )
        };

        match hr {
            HRESULT::S_OK => Ok(code_info),
            _ => Err(hr),
        }
    }
    fn get_function_from_ip_2(&self, ip: LPCBYTE) -> Result<FunctionAndRejit, HRESULT> {
        let mut function_id = MaybeUninit::uninit();
        let mut rejit_id = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetFunctionFromIP2(ip, function_id.as_mut_ptr(), rejit_id.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let function_id = unsafe { function_id.assume_init() };
                let rejit_id = unsafe { rejit_id.assume_init() };
                Ok(FunctionAndRejit { function_id, rejit_id })
            }
            _ => Err(hr),
        }
    }
    fn get_rejit_ids(&self, function_id: FunctionID) -> Result<Vec<ReJITID>, HRESULT> {
        let mut rejit_ids_buffer_length = MaybeUninit::uninit();
        unsafe { self.info().GetReJITIDs(function_id, 0, rejit_ids_buffer_length.as_mut_ptr(), ptr::null_mut()) };

        let rejit_ids_buffer_length = unsafe { rejit_ids_buffer_length.assume_init() };
        let mut rejit_ids = Vec::<ReJITID>::with_capacity(rejit_ids_buffer_length as usize);
        unsafe { rejit_ids.set_len(rejit_ids_buffer_length as usize) };

        let mut rejit_ids_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetReJITIDs(function_id, rejit_ids_buffer_length, rejit_ids_length.as_mut_ptr(), rejit_ids.as_mut_ptr())
        };

        match hr {
            HRESULT::S_OK => Ok(rejit_ids),
            _ => Err(hr),
        }
    }
    fn get_il_to_native_mapping_2(&self, function_id: FunctionID, rejit_id: ReJITID) -> Result<Vec<COR_DEBUG_IL_TO_NATIVE_MAP>, HRESULT> {
        let mut map_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info()
                .GetILToNativeMapping2(function_id, rejit_id, 0, map_buffer_length.as_mut_ptr(), ptr::null_mut())
        };

        let map_buffer_length = unsafe { map_buffer_length.assume_init() };
        let mut map = Vec::<COR_DEBUG_IL_TO_NATIVE_MAP>::with_capacity(map_buffer_length as usize);
        unsafe { map.set_len(map_buffer_length as usize) };
        let mut map_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetILToNativeMapping2(function_id, rejit_id, map_buffer_length, map_length.as_mut_ptr(), map.as_mut_ptr())
        };
        match hr {
            HRESULT::S_OK => Ok(map),
            _ => Err(hr),
        }
    }
    fn enum_jited_functions_2(&self) -> Result<&mut CorProfilerFunctionEnum, HRESULT> {
        let mut function_enum = MaybeUninit::uninit();
        let hr = unsafe { self.info().EnumJITedFunctions2(function_enum.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let function_enum = unsafe { function_enum.assume_init().as_mut().unwrap() };
                Ok(function_enum)
            }
            _ => Err(hr),
        }
    }
    fn get_object_size_2(&self, object_id: ObjectID) -> Result<usize, HRESULT> {
        let mut object_size = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetObjectSize2(object_id, object_size.as_mut_ptr()) };

        match hr {
            HRESULT::S_OK => {
                let object_size = unsafe { object_size.assume_init() };
                Ok(object_size)
            }
            _ => Err(hr),
        }
    }
}
impl CorProfilerInfo5 for ClrProfilerInfo {
    fn get_event_mask_2(&self) -> Result<EventMask2, HRESULT> {
        let mut events_low = MaybeUninit::uninit();
        let mut events_high = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetEventMask2(events_low.as_mut_ptr(), events_high.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let events_low = unsafe { events_low.assume_init() };
                let events_high = unsafe { events_high.assume_init() };
                Ok(EventMask2 {
                    events_low: COR_PRF_MONITOR::from_bits(events_low).unwrap(),
                    events_high: COR_PRF_HIGH_MONITOR::from_bits(events_high).unwrap(),
                })
            }
            _ => Err(hr),
        }
    }
    fn set_event_mask_2(&self, events_low: COR_PRF_MONITOR, events_high: COR_PRF_HIGH_MONITOR) -> Result<(), HRESULT> {
        let events_low = events_low.bits();
        let events_high = events_high.bits();
        let hr = unsafe { self.info().SetEventMask2(events_low, events_high) };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
}
impl CorProfilerInfo6 for ClrProfilerInfo {
    fn enum_ngen_module_methods_inlining_this_method(
        &self,
        inliners_module_id: ModuleID,
        inlinee_module_id: ModuleID,
        inlinee_method_id: mdMethodDef,
    ) -> Result<EnumNgenModuleMethodsInliningThisMethod, HRESULT> {
        let mut incomplete_data = MaybeUninit::uninit();
        let mut method_enum = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().EnumNgenModuleMethodsInliningThisMethod(
                inliners_module_id,
                inlinee_module_id,
                inlinee_method_id,
                incomplete_data.as_mut_ptr(),
                method_enum.as_mut_ptr(),
            )
        };
        match hr {
            HRESULT::S_OK => {
                let incomplete_data = unsafe { incomplete_data.assume_init() };
                let incomplete_data = incomplete_data > 0;
                let method_enum = unsafe { method_enum.assume_init().as_mut().unwrap() };
                Ok(EnumNgenModuleMethodsInliningThisMethod { incomplete_data, method_enum })
            }
            _ => Err(hr),
        }
    }
}
impl CorProfilerInfo7 for ClrProfilerInfo {
    fn apply_metadata(&self, module_id: ModuleID) -> Result<(), HRESULT> {
        let hr = unsafe { self.info().ApplyMetaData(module_id) };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn get_in_memory_symbols_length(&self, module_id: ModuleID) -> Result<u32, HRESULT> {
        let mut symbol_bytes = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetInMemorySymbolsLength(module_id, symbol_bytes.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let symbol_bytes = unsafe { symbol_bytes.assume_init() };
                Ok(symbol_bytes)
            }
            _ => Err(hr),
        }
    }
    fn read_in_memory_symbols(&self, module_id: ModuleID, symbols_read_offset: u32, count_symbol_bytes: u32) -> Result<Vec<BYTE>, HRESULT> {
        let mut buffer = Vec::<BYTE>::with_capacity((count_symbol_bytes + 1024) as usize);
        let mut symbol_bytes_read = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().ReadInMemorySymbols(
                module_id,
                symbols_read_offset,
                buffer.as_mut_ptr(),
                count_symbol_bytes,
                symbol_bytes_read.as_mut_ptr(),
            )
        };
        match hr {
            HRESULT::S_OK => {
                let symbol_bytes_read = unsafe { symbol_bytes_read.assume_init() };
                unsafe { buffer.set_len(symbol_bytes_read as usize) };
                Ok(buffer)
            }
            _ => Err(hr),
        }
    }
}
impl CorProfilerInfo8 for ClrProfilerInfo {
    fn is_function_dynamic(&self, function_id: FunctionID) -> Result<bool, HRESULT> {
        let mut is_dynamic = MaybeUninit::uninit();
        let hr = unsafe { self.info().IsFunctionDynamic(function_id, is_dynamic.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let is_dynamic = unsafe { is_dynamic.assume_init() };
                let is_dynamic = is_dynamic > 0;
                Ok(is_dynamic)
            }
            _ => Err(hr),
        }
    }
    fn get_function_from_ip_3(&self, ip: LPCBYTE) -> Result<FunctionAndRejit, HRESULT> {
        let mut function_id = MaybeUninit::uninit();
        let mut rejit_id = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetFunctionFromIP3(ip, function_id.as_mut_ptr(), rejit_id.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let function_id = unsafe { function_id.assume_init() };
                let rejit_id = unsafe { rejit_id.assume_init() };
                Ok(FunctionAndRejit { function_id, rejit_id })
            }
            _ => Err(hr),
        }
    }
    fn get_dynamic_function_info(&self, function_id: FunctionID) -> Result<DynamicFunctionInfo, HRESULT> {
        let mut name_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info().GetDynamicFunctionInfo(
                function_id,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                name_buffer_length.as_mut_ptr(),
                ptr::null_mut(),
            )
        };

        let name_buffer_length = unsafe { name_buffer_length.assume_init() };
        let mut name_buffer = Vec::<WCHAR>::with_capacity(name_buffer_length as usize);
        unsafe { name_buffer.set_len(name_buffer_length as usize) };

        let mut name_length = MaybeUninit::uninit();
        let mut module_id = MaybeUninit::uninit();
        let mut sig = MaybeUninit::uninit();
        let mut sig_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetDynamicFunctionInfo(
                function_id,
                module_id.as_mut_ptr(),
                sig.as_mut_ptr(),
                sig_length.as_mut_ptr(),
                name_buffer_length,
                name_length.as_mut_ptr(),
                name_buffer.as_mut_ptr(),
            )
        };
        match hr {
            HRESULT::S_OK => {
                let module_id = unsafe { module_id.assume_init() };
                let sig = unsafe { sig.assume_init() };
                let sig_length = unsafe { sig_length.assume_init() };
                let name = U16CString::from_vec_with_nul(name_buffer).unwrap().to_string_lossy();
                Ok(DynamicFunctionInfo {
                    module_id,
                    sig,
                    sig_length,
                    name,
                })
            }
            _ => Err(hr),
        }
    }
}
impl CorProfilerInfo9 for ClrProfilerInfo {
    fn get_native_code_start_addresses(&self, function_id: FunctionID, rejit_id: ReJITID) -> Result<Vec<UINT_PTR>, HRESULT> {
        let mut addresses_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info()
                .GetNativeCodeStartAddresses(function_id, rejit_id, 0, addresses_buffer_length.as_mut_ptr(), ptr::null_mut())
        };

        let addresses_buffer_length = unsafe { addresses_buffer_length.assume_init() };
        let mut addresses_buffer = Vec::<UINT_PTR>::with_capacity(addresses_buffer_length as usize);
        unsafe { addresses_buffer.set_len(addresses_buffer_length as usize) };

        let mut addresses_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetNativeCodeStartAddresses(
                function_id,
                rejit_id,
                addresses_buffer_length,
                addresses_length.as_mut_ptr(),
                addresses_buffer.as_mut_ptr(),
            )
        };
        match hr {
            HRESULT::S_OK => Ok(addresses_buffer),
            _ => Err(hr),
        }
    }
    fn get_il_to_native_mapping_3(&self, native_code_start_address: UINT_PTR) -> Result<Vec<COR_DEBUG_IL_TO_NATIVE_MAP>, HRESULT> {
        let mut map_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info()
                .GetILToNativeMapping3(native_code_start_address, 0, map_buffer_length.as_mut_ptr(), ptr::null_mut())
        };

        let map_buffer_length = unsafe { map_buffer_length.assume_init() };
        let mut map = Vec::<COR_DEBUG_IL_TO_NATIVE_MAP>::with_capacity(map_buffer_length as usize);
        unsafe { map.set_len(map_buffer_length as usize) };
        let mut map_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info()
                .GetILToNativeMapping3(native_code_start_address, map_buffer_length, map_length.as_mut_ptr(), map.as_mut_ptr())
        };
        match hr {
            HRESULT::S_OK => Ok(map),
            _ => Err(hr),
        }
    }
    fn get_code_info_4(&self, native_code_start_address: UINT_PTR) -> Result<Vec<COR_PRF_CODE_INFO>, HRESULT> {
        let mut code_info_buffer_length = MaybeUninit::uninit();
        unsafe {
            self.info()
                .GetCodeInfo4(native_code_start_address, 0, code_info_buffer_length.as_mut_ptr(), ptr::null_mut())
        };

        let code_info_buffer_length = unsafe { code_info_buffer_length.assume_init() };
        let mut code_info = Vec::<COR_PRF_CODE_INFO>::with_capacity(code_info_buffer_length as usize);
        unsafe { code_info.set_len(code_info_buffer_length as usize) };

        let mut code_info_length = MaybeUninit::uninit();
        let hr = unsafe {
            self.info().GetCodeInfo4(
                native_code_start_address,
                code_info_buffer_length,
                code_info_length.as_mut_ptr(),
                code_info.as_mut_ptr(),
            )
        };

        match hr {
            HRESULT::S_OK => Ok(code_info),
            _ => Err(hr),
        }
    }
}
impl CorProfilerInfo10 for ClrProfilerInfo {
    fn enumerate_object_references(&self, object_id: ObjectID, callback: ObjectReferenceCallback, client_data: *mut std::ffi::c_void) -> Result<(), HRESULT> {
        let hr = unsafe { self.info().EnumerateObjectReferences(object_id, callback, client_data) };

        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn is_frozen_object(&self, object_id: ObjectID) -> Result<bool, HRESULT> {
        let mut is_frozen = MaybeUninit::uninit();
        let hr = unsafe { self.info().IsFrozenObject(object_id, is_frozen.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let is_frozen = unsafe { is_frozen.assume_init() };
                let is_frozen = is_frozen > 0;
                Ok(is_frozen)
            }
            _ => Err(hr),
        }
    }
    fn get_loh_object_size_threshold(&self) -> Result<u32, HRESULT> {
        let mut threshold = MaybeUninit::uninit();
        let hr = unsafe { self.info().GetLOHObjectSizeThreshold(threshold.as_mut_ptr()) };
        match hr {
            HRESULT::S_OK => {
                let threshold = unsafe { threshold.assume_init() };
                Ok(threshold)
            }
            _ => Err(hr),
        }
    }
    fn request_rejit_with_inliners(
        &self,
        dw_rejit_flags: COR_PRF_REJIT_FLAGS,
        module_ids: &[ModuleID],
        method_ids: &[mdMethodDef], // TODO: Maybe we want the pairs to be actual tuples. Simple zip op.
    ) -> Result<(), HRESULT> {
        let dw_rejit_flags = dw_rejit_flags.bits();
        let methods_length = module_ids.len() as u32;
        let module_ids = module_ids.as_ptr();
        let method_ids = method_ids.as_ptr();
        let hr = unsafe { self.info().RequestReJITWithInliners(dw_rejit_flags, methods_length, module_ids, method_ids) };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn suspend_runtime(&self) -> Result<(), HRESULT> {
        let hr = unsafe { self.info().SuspendRuntime() };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
    fn resume_runtime(&self) -> Result<(), HRESULT> {
        let hr = unsafe { self.info().ResumeRuntime() };
        match hr {
            HRESULT::S_OK => Ok(()),
            _ => Err(hr),
        }
    }
}
