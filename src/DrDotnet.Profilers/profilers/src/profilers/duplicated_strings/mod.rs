use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use profiling_api::*;
use uuid::Uuid;
use std::thread;
use profiling_api::ffi::{ClassID, HRESULT, ObjectID};

use crate::report::*;
use crate::profilers::*;

#[derive(Default)]
pub struct DuplicatedStringsProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    object_to_class: HashMap<ffi::ObjectID, ffi::ClassID>,
    nb_of_occurrences_by_string: HashMap<String, u64>,
    record_object_references: bool
}

impl Profiler for DuplicatedStringsProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("bdaba522-104c-4343-8952-036bed81527d").unwrap(),
            name: "Duplicated Strings".to_owned(),
            description: "For now, just duplicated strings and their occurence".to_owned(),
            is_released: true,
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl CorProfilerCallback for DuplicatedStringsProfiler
{
    fn object_references(&mut self, object_id: ObjectID, class_id: ClassID, _object_ref_ids: &[ObjectID]) -> Result<(), HRESULT> {
        
        if !self.record_object_references {
            // Early return if we received an event before the forced GC started
            return Ok(());
        }
        
        let pinfo = self.profiler_info();
        let type_name = match pinfo.get_class_id_info(class_id) {
            Ok(class_info) => extensions::get_type_name(pinfo, class_info.module_id, class_info.token),
            _ => "unknown".to_owned()
        };
        // debug!("Object id: {}, Class id: {}, Type name: {}", object_id, class_id, type_name);
        
        // Keep ref of string only
        if type_name == "System.String" {
            self.object_to_class.insert(object_id, class_id);
        }

        Ok(())
    }
}

impl CorProfilerCallback2 for DuplicatedStringsProfiler
{
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT>
    {
        info!("GC started on gen {} for reason {:?}", extensions::get_gc_gen(&generation_collected), reason);
        
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
        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_NONE) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:x}", hresult)
        }

        let str_layout = match self.profiler_info().get_string_layout_2() {
            Ok(str_layout) => str_layout,
            Err(hresult) => {
                error!("Error getting string layout: {:x}", hresult);
                return Err(hresult);
            }
        };
        
        // Process the recorded objects
        for (object_id, class_id) in self.object_to_class.iter() {

            let str = get_string_value(&str_layout, object_id);
        
            debug!("String value: {}", str);
        }

        // We're done, we can detach :)
        let profiler_info = self.profiler_info().clone();
        profiler_info.request_profiler_detach(3000).ok();
        
        Ok(())
    }
}

impl CorProfilerCallback3 for DuplicatedStringsProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        self.profiler_info = Some(profiler_info);
        
        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC) {
            Ok(_) => info!("Succeed to register profiler for COR_PRF_MONITOR_GC events"),
            Err(hresult) => error!("Error setting event mask: {:x}", hresult)
        }

        match init_session(client_data, client_data_length) {
            Ok(uuid) => {
                self.session_id = uuid;
                Ok(())
            },
            Err(err) => Err(err)
        }
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT>
    {
        // The ForceGC method must be called only from a thread that does not have any profiler callbacks on its stack. 
        // https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-forcegc-method
        let p_clone = self.profiler_info().clone();
        let _ = thread::spawn(move || {
            debug!("Force GC");
            match p_clone.force_gc() {
                Ok(_) => debug!("GC Forced!"),
                Err(hresult) => error!("Error forcing GC: {:x}", hresult)
            };
        }).join();
        
        // Security timeout
        detach_after_duration::<DuplicatedStringsProfiler>(&self, 360, None);

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        let session = Session::get_session(self.session_id, AllocationByClassProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

        report.write_line(format!("# Duplicate strings Report"));

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