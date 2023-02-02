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

pub unsafe fn get_string_value(str_layout: &StringLayout, object_id: &ObjectID) -> String {

    // let ptr = (*object_id as *const c_char).offset(str_layout.buffer_offset as isize);
    // let c_str: &CStr = unsafe { CStr::from_ptr(ptr) };
    // c_str.to_str().unwrap().to_owned()

    // https://learn.microsoft.com/en-us/archive/blogs/davbr/when-is-it-safe-to-use-objectids
    
    let ptr = (*object_id as *const u16).offset(str_layout.buffer_offset as isize);
    // let len = (*object_id as *const DWORD).offset(str_layout.string_length_offset as isize);
    let len =  (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);
    String::from_utf16(slice).unwrap().to_owned()
}