
use crate::ffi::*;
use crate::*;

pub fn get_type_name(info: &ProfilerInfo, module_id: usize, td: mdTypeDef) -> String {
    match info.get_module_metadata(module_id, CorOpenFlags::ofRead) {
        Ok(metadata) =>
        match metadata.get_type_def_props(td) {
            Ok(type_props) => type_props.name,
            _ => "unknown4".to_owned()
        }, 
        _ => "unknown3".to_owned()
    }
}

pub unsafe fn get_method_name(info: &ProfilerInfo, method_id: usize) -> String {
    match info.get_token_and_metadata_from_function(method_id) {
        Ok(f) =>
        match f.metadata_import.get_method_props(f.token) {
            Ok(method_props) => method_props.name,
            _ => "unknown4".to_owned()
        },
        _ => "unmanaged".to_owned()
    }
}