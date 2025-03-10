use std::cell::RefCell;
use std::collections::HashMap;

use crate::api::*;
use crate::ffi::*;

#[derive(Default)]
pub struct CachedNameResolver {
    classes_cache: RefCell<HashMap<ClassID, String>>,
    functions_cache: RefCell<HashMap<(FunctionID, COR_PRF_FRAME_INFO), String>>,
    info: ClrProfilerInfo,
}

pub trait NameResolver {
    fn get_full_method_name(&self, method_id: FunctionID, class_id: ClassID) -> String;
    fn get_class_name(&self, class_id: ClassID) -> String;
}

impl CachedNameResolver {
    pub fn new(info: ClrProfilerInfo) -> Self {
        CachedNameResolver {
            classes_cache: RefCell::new(HashMap::new()),
            functions_cache: RefCell::new(HashMap::new()),
            info: info,
        }
    }
}

impl NameResolver for CachedNameResolver {
    // Returns a method name and the type where it is defined (namespaced) for a given FunctionID
    fn get_full_method_name(&self, method_id: FunctionID, class_id: ClassID) -> String {
        self.functions_cache
            .borrow_mut()
            .entry((method_id, class_id))
            .or_insert_with(|| self.info.get_full_method_name(method_id, class_id))
            .clone()
    }

    // Returns a class name (namespaced) for a given ClassID
    fn get_class_name(&self, class_id: ClassID) -> String {
        // Check if the key exists in the map
        {
            let map_borrowed = self.classes_cache.borrow();
            if let Some(value) = map_borrowed.get(&class_id) {
                return value.clone();
            }
        }

        let name = self.info.get_class_name(class_id);

        {
            let mut map_borrowed = self.classes_cache.borrow_mut();
            map_borrowed.insert(class_id, name.clone());
        }

        name
    }
}
