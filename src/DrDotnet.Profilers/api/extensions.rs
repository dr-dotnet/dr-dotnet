use std::ffi::{c_char, CStr};
use std::ptr::null;
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

pub fn get_string_value(info: &ProfilerInfo, object_id: &ObjectID) -> String {
    match info.get_string_layout_2() {
        Ok(str_layout) => {
            let ptr_size = std::mem::size_of::<usize>();
            let str_ptr = match ptr_size {
                4 => (((*object_id) as u32) + str_layout.buffer_offset) as *const c_char,
                8 => (((*object_id) as u64) + (str_layout.buffer_offset as u64)) as *const c_char,
                _ => panic!("pointer size not handled")
            };
            let c_str: &CStr = unsafe {
                CStr::from_ptr(str_ptr) 
            };
            let str_slice: &str = c_str.to_str().unwrap();
            str_slice.to_string().to_owned()
        },
        Err(hresult) => {
            warn!("get_string_value({}) failed (0x{})", object_id, format!("{:01$x}", hresult, 8));
            "unknown_0006".to_owned()
        }
    }
}