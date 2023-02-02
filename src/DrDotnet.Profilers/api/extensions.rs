use std::ffi::{c_char, CStr, OsString};
use std::ptr::null;
use std::slice;
use crate::ffi::*;
use crate::*;

pub fn get_type_name(info: &ProfilerInfo, module_id: ModuleID, td: mdTypeDef) -> String {
    match info.get_module_metadata(module_id, CorOpenFlags::ofRead) {
        Ok(metadata) =>
        match metadata.get_type_def_props(td) {
            Ok(type_props) => type_props.name,
            Err(hresult) => {
                warn!("metadata.get_type_def_props({}) failed (0x{})", td, format!("{:01$x}", hresult, 8));
                "unknown_0002".to_owned()
            }
        }, 
        Err(hresult) => {
            warn!("info.get_module_metadata({}) failed (0x{})", module_id, format!("{:01$x}", hresult, 8));
            "unknown_0001".to_owned()
        }
    }
}

pub unsafe fn get_method_name(info: &ProfilerInfo, method_id: FunctionID) -> String {
    match info.get_token_and_metadata_from_function(method_id) {
        Ok(f) =>
        match f.metadata_import.get_method_props(f.token) {
            Ok(method_props) => method_props.name,
            Err(hresult) => {
                warn!("metadata_import.get_method_props({}) failed (0x{})", f.token, format!("{:01$x}", hresult, 8));
                "unknown_0004".to_owned()
            }
        },
        Err(hresult) => {
            warn!("info.get_token_and_metadata_from_function({}) failed (0x{})", method_id, format!("{:01$x}", hresult, 8));
            "unknown_0003".to_owned()
        }
    }
}

pub unsafe fn get_full_method_name(info: &ProfilerInfo, method_id: FunctionID) -> String {

    match info.get_function_info(method_id) {
        Ok(function_info) =>
        match info.get_token_and_metadata_from_function(method_id) {
            Ok(f) =>
            match f.metadata_import.get_method_props(f.token) {
                Ok(method_props) => format!("{}.{}", get_type_name(info, function_info.module_id, method_props.class_token), method_props.name),
                Err(hresult) => {
                    warn!("metadata_import.get_method_props({}) failed (0x{})", f.token, format!("{:01$x}", hresult, 8));
                    "unknown_0004".to_owned()
                }
            },
            Err(hresult) => {
                warn!("info.get_token_and_metadata_from_function({}) failed (0x{})", method_id, format!("{:01$x}", hresult, 8));
                "unknown_0003".to_owned()
            }
        },
        Err(hresult) => {
            warn!("info.get_function_info({}) failed (0x{})", method_id, format!("{:01$x}", hresult, 8));
            "unknown_0002".to_owned()
        }
    }
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