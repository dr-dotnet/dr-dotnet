use std::collections::HashMap;
use std::thread;
use itertools::Itertools;
use widestring::U16CString;

use crate::api::*;
use crate::api::ffi::{ClassID, HRESULT, ObjectID};
use crate::macros::*;
use crate::profilers::*;

#[derive(Default)]
pub struct DuplicatedStringsProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    string_object_ids: Vec<ObjectID>,
    str_counts: HashMap<String, u64>,
    string_class_id: Option<ClassID>,
    record_object_references: bool
}

impl DuplicatedStringsProfiler {
    pub fn count_utf16_bytes(str: &str) -> Result<usize, ()> {
        match U16CString::from_str(str) {
            Ok(str_utf16) => Ok(str_utf16.len() * 2),
            Err(_) => Err(())
        }
    }
}

impl Profiler for DuplicatedStringsProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "bdaba522-104c-4343-8952-036bed81527d".to_owned(),
            name: "Duplicated Strings".to_owned(),
            description: "List strings object with the same value by count".to_owned(),
            is_released: true,
            parameters: vec![
                ProfilerParameter {
                    name: "Top".to_owned(),
                    key: "top_count".to_owned(),
                    description: "The number of string to list in the report.".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "100".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter {
                    name: "Maximum String Size".to_owned(),
                    key: "max_string_display_size".to_owned(),
                    description: "The maximum number of characters to display for a given string.".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "128".to_owned(),
                    ..std::default::Default::default()
                },
            ],
            ..std::default::Default::default()
        }
    }
}

impl CorProfilerCallback for DuplicatedStringsProfiler {
    
    fn object_references(&mut self, object_id: ObjectID, class_id: ClassID, _object_ref_ids: &[ObjectID]) -> Result<(), HRESULT> {
        
        if !self.record_object_references {
            // Early return if we received an event before the forced GC started
            return Ok(());
        }
        
        // We store the string class ID once we found it once so that we don't have to parse the type name every time
        match self.string_class_id {
            Some(id) => {
                if id == class_id {
                    self.string_object_ids.push(object_id);
                }
            },
            None => {
                let clr = self.clr();
                let type_name = match clr.get_class_id_info(class_id) {
                    Ok(class_info) => clr.get_type_name(class_info.module_id, class_info.token),
                    _ => "unknown".to_owned()
                };

                if type_name == "System.String" {
                    self.string_class_id = Option::Some(class_id);
                    return self.object_references(object_id, class_id, _object_ref_ids);
                }
            }
        }

        Ok(())
    }
}

impl CorProfilerCallback2 for DuplicatedStringsProfiler {
    
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT> {
        info!("GC started on gen {} for reason {:?}", ClrProfilerInfo::get_gc_gen(&generation_collected), reason);
        
        // Start recording object 
        if reason == ffi::COR_PRF_GC_REASON::COR_PRF_GC_INDUCED 
            && !self.record_object_references {
            self.record_object_references = true;
        }

        Ok(())
    }
    
    fn garbage_collection_finished(&mut self) -> Result<(), HRESULT> {
        info!("GC finished");
        self.record_object_references = false;

        // Disable profiling to free some resources
        match self.clr().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_NONE) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:?}", hresult)
        }

        let str_layout = match self.clr().get_string_layout_2() {
            Ok(str_layout) => str_layout,
            Err(hresult) => {
                error!("Error getting string layout: {:?}", hresult);
                return Err(hresult);
            }
        };
        
        // Process the recorded objects
        for object_id in self.string_object_ids.iter() {
            // Get string value and increment it's count
            let str = ClrProfilerInfo::get_string_value(&str_layout, object_id);
            let count = self.str_counts.entry(str).or_insert(0);
            *count += 1;
        }

        // We're done, we can detach :)
        let profiler_info = self.clr().clone();
        profiler_info.request_profiler_detach(3000).ok();
        
        Ok(())
    }
}

impl CorProfilerCallback3 for DuplicatedStringsProfiler {
    
    fn initialize_for_attach(&mut self, profiler_info: ClrProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT> {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC, None, profiler_info, client_data, client_data_length)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        // The ForceGC method must be called only from a thread that does not have any profiler callbacks on its stack. 
        // https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-forcegc-method
        let p_clone = self.clr().clone();
        let _ = thread::spawn(move || {
            debug!("Force GC");
            match p_clone.force_gc() {
                Ok(_) => debug!("GC Forced!"),
                Err(hresult) => error!("Error forcing GC: {:?}", hresult)
            };
        }).join();
        
        // Security timeout
        detach_after_duration::<DuplicatedStringsProfiler>(&self, 360, None);

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT> {
        let mut report = self.session_info.create_report("summary.md".to_owned());

        report.write_line(format!("# Duplicated Strings Report"));

        let count_of_str_to_print = self.session_info().get_parameter::<usize>("top_count").unwrap();
        let max_string_display_size = self.session_info().get_parameter::<usize>("max_string_display_size").unwrap();

        report.write_line(format!("Number of occurrences | Value | Wasted bytes"));
        report.write_line(format!(":- | :- | -:"));

        let mut i = 0;
        let mut total_wasted_bytes: u64 = 0;
        
        for (value, count) in self.str_counts.iter().sorted_by(|a, b| a.1.cmp(b.1).reverse()) {
            // Wasted bytes = (occurrences - 1) * (utf-16 size (dotnet uses utf-16) + length on 4 bytes)
            let wasted_bytes = match DuplicatedStringsProfiler::count_utf16_bytes(value) {
                Ok(size) => (count - 1) * (size as u64 + 4),
                Err(()) => 0
            };
            total_wasted_bytes = total_wasted_bytes + wasted_bytes;
            if i < count_of_str_to_print {
                i = i + 1;
                let mut truncated_string: String = if value.len() > max_string_display_size {
                    let mut t_str = value.clone();
                    t_str.truncate(max_string_display_size);
                    t_str + "..."
                } else {
                    value.to_string()
                };

                // Replace EOT characters like newlines, tabs, ACK, EOT, NUL, ...
                truncated_string = truncated_string.replace(|c: char| c < 17 as char, "�");
                
                report.write_line(format!("{} | `{}` | {}", count, truncated_string, if wasted_bytes > 0 { wasted_bytes.to_string() } else { "???".to_string() }));
            }
        }

        report.new_line();
        report.write_line(format!("Total wasted bytes: {}", total_wasted_bytes));

        info!("Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for DuplicatedStringsProfiler {}
impl CorProfilerCallback5 for DuplicatedStringsProfiler {}
impl CorProfilerCallback6 for DuplicatedStringsProfiler {}
impl CorProfilerCallback7 for DuplicatedStringsProfiler {}
impl CorProfilerCallback8 for DuplicatedStringsProfiler {}
impl CorProfilerCallback9 for DuplicatedStringsProfiler {}

#[cfg(test)]
mod tests {
    use crate::profilers::DuplicatedStringsProfiler;

    #[test]
    fn count_string_utf16_bytes_ascii() {
        // Each ascii characters take 2 bytes when utf-16 encoded
        assert_eq!(DuplicatedStringsProfiler::count_utf16_bytes("1234").unwrap(), 8);
    }

    #[test]
    fn count_string_utf16_bytes_unicode() {
        // Special unicode characters can take more than 2 bytes when utf-16 encoded
        assert_eq!(DuplicatedStringsProfiler::count_utf16_bytes("🐶❌😬😈").unwrap(), 14);
    }
}