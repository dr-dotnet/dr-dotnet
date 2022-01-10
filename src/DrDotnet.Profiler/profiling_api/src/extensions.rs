
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