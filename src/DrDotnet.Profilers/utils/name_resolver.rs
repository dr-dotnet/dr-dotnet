use std::collections::HashMap;
use itertools::Itertools;
use std::cell::RefCell;

use crate::api::*;
use crate::ffi::*;

#[derive(Default)]
pub struct NameResolver {
    classes_cache: RefCell<HashMap<ClassID, String>>,
    functions_cache: RefCell<HashMap<FunctionID, String>>,
    info: ClrProfilerInfo,
}

impl NameResolver {
    pub fn new(info: ClrProfilerInfo) -> Self {
        NameResolver {
            classes_cache: RefCell::new(HashMap::new()),
            functions_cache: RefCell::new(HashMap::new()),
            info: info
        }
    }

    // Returns a method name for a given FunctionID
    pub fn get_method_name(&self, method_id: FunctionID) -> String {
        match self.info.get_token_and_metadata_from_function(method_id) {
            Ok(f) =>
            match f.metadata_import.get_method_props(f.token) {
                Ok(method_props) => method_props.name,
                Err(hresult) => {
                    warn!("metadata_import.get_method_props({}) failed ({:?})", f.token, hresult);
                    "unknown".to_owned()
                }
            },
            Err(hresult) => {
                warn!("info.get_token_and_metadata_from_function({}) failed ({:?})", method_id, hresult);
                "unknown".to_owned()
            }
        }
    }

    // Returns a method name and the type where it is defined (namespaced) for a given FunctionID
    pub fn get_full_method_name(&self, method_id: FunctionID) -> String {
        match self.info.get_function_info(method_id) {
            Ok(function_info) =>
            match self.info.get_token_and_metadata_from_function(method_id) {
                Ok(f) =>
                match f.metadata_import.get_method_props(f.token) {
                    Ok(method_props) => format!("{}.{}", self.get_type_name(function_info.module_id, method_props.class_token), method_props.name),
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
    }

    // Return the name of type (with its namespace)
    fn get_type_name(&self, module_id: ModuleID, td: mdTypeDef) -> String {

        impl NameResolver {
            fn handle_nesting(&self, type_props: TypeProps, metadata: &MetadataImport, td: mdTypeDef, module_id: ModuleID) -> String {
                if type_props.type_def_flags.is_nested() {
                    match metadata.get_nested_class_props(td) {
                        Ok(nested_td) => {
                            let nesting_type_name = self.get_type_name(module_id, nested_td);
                            format!("{}.{}", nesting_type_name, type_props.name)
                        },
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

        match self.info.get_module_metadata(module_id, CorOpenFlags::ofRead) {
            Ok(metadata) => match metadata.get_type_def_props(td) {
                Ok(type_props) => {
                    // If type is nested in another type, recursively get the name of the parent type to prefix it
                    let type_name = self.handle_nesting(type_props, &metadata, td, module_id);
                    type_name
                },
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

    // Returns a class name (namespaced) for a given ClassID
    pub fn get_class_name(&self, class_id: ClassID) -> String {
        
        impl NameResolver {
            // If the type is an array, recursively drill until the base object type is found
            fn get_inner_type(&self, class_id: ClassID, array_dimension: &mut usize) -> ClassID {
                // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-isarrayclass-method
                match self.info.is_array_class(class_id) {
                    Ok(array_class_info) => {
                        *array_dimension = *array_dimension + 1;
                        // TODO: Handle array_class_info.rank
                        if let Some(element_class_id) = array_class_info.element_class_id {
                            self.get_inner_type(element_class_id, array_dimension)
                        } else {
                            error!("No element class id for array class object");
                            class_id
                        }
                    },
                    Err(_) => class_id,
                }
            }

            fn handle_generics(&self, type_name: String, class_info: &ClassInfo2) -> String {

                let total_generic_types = class_info.type_args.len();

                if total_generic_types == 0 {
                    return type_name;
                }

                // Before calling this function, generic types in a type name a hidden behind `N
                // where N is a number of generic parameters.
                // This method iterates all occurrences of `N in a type name (there can be more than one,
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
                            let arg_names = (0..args_count).into_iter().map(|i| {
                                let arg_class_id = class_info.type_args[current_generic_type_index];
                                let arg_name = self.get_class_name(arg_class_id);
                                current_generic_type_index += 1;
                                return arg_name;
                            }).join(", ");

                            // Surrounds generic arguments with < > in html
                            outstring.push_str(&format!("&lt;{}&gt;", arg_names));
                        },
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
 
        // Check if the key exists in the map
        {
            let map_borrowed = self.classes_cache.borrow();
            if let Some(value) = map_borrowed.get(&class_id) {
                return value.clone();
            }
        }

        // If the key doesn't exist, calculate the value
        let mut array_dimension = 0;
        let class_id = self.get_inner_type(class_id, &mut array_dimension);

        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-getclassidinfo-method
        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo2-getclassidinfo2-method
        let mut name = match self.info.get_class_id_info_2(class_id) {
            Ok(class_info) => {
                let name = self.get_type_name(class_info.module_id, class_info.token);
                let name = self.handle_generics(name, &class_info);
                name
            } 
            _ => "unknown".to_owned()
        };

        // Append array symbols []
        if array_dimension > 0 {
            name.reserve(array_dimension * 2);
            for _ in 0..array_dimension {
                name.push_str("[]");
            }
        }

        {
            let mut map_borrowed = self.classes_cache.borrow_mut();
            map_borrowed.insert(class_id, name.clone());
        }

        name
    }
}