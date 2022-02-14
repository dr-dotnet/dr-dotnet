use dashmap::DashMap;
use profiling_api::*;
use uuid::Uuid;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicIsize, Ordering};

use crate::report::*;
use crate::profilers::*;

pub struct AllocationByClassProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    allocations_by_class: DashMap<String, AtomicIsize>,
    collections: AtomicIsize,
}

impl Profiler for AllocationByClassProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E84").unwrap(),
            name: "Allocations by Class".to_owned(),
            description: "For now, just allocations by class".to_owned(),
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl Clone for AllocationByClassProfiler {
    fn clone(&self) -> Self { 
        AllocationByClassProfiler {
            profiler_info: self.profiler_info.clone(),
            session_id: self.session_id.clone(),
            allocations_by_class: DashMap::new(),
            collections: AtomicIsize::new(0)
        }
    }
}

impl ClrProfiler for AllocationByClassProfiler {
    fn new() -> AllocationByClassProfiler {
        AllocationByClassProfiler {
            profiler_info: None,
            session_id: Uuid::default(),
            allocations_by_class: DashMap::new(),
            collections: AtomicIsize::new(0)
        }
    }
}

impl CorProfilerCallback for AllocationByClassProfiler
{
    fn objects_allocated_by_class(&mut self, class_ids: &[ffi::ClassID], num_objects: &[u32]) -> Result<(), ffi::HRESULT>
    {
        self.allocations_by_class.insert("test".to_owned(), AtomicIsize::new(123));

        for i in 0..class_ids.len() {
            
            let pinfo = self.profiler_info();
            let name = match pinfo.get_class_id_info(class_ids[i]) {
                Ok(class_info) => extensions::get_type_name(pinfo, class_info.module_id, class_info.token),
                _ => "unknown2".to_owned()
            };

            let key = name;
            let count = num_objects[i] as isize;

            match self.allocations_by_class.get_mut(&key) {
                Some(pair) => { pair.value().fetch_add(count, Ordering::Relaxed); },
                None => { self.allocations_by_class.insert(key, AtomicIsize::new(count)); },
            }
        }

        Ok(())
    }
}

impl CorProfilerCallback2 for AllocationByClassProfiler
{
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT>
    {
        self.collections.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }
}

impl CorProfilerCallback3 for AllocationByClassProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        println!("[profiler] Initialize with attach");
        self.profiler_info = Some(profiler_info);
        
        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC) {
            Ok(_) => (),
            Err(hresult) => println!("Error setting event mask: {:x}", hresult)
        }

        unsafe {
            let cstr = std::ffi::CStr::from_ptr(client_data as *const _).to_string_lossy();
            self.session_id = Uuid::parse_str(&cstr).unwrap();
        }

        println!("[profiler] Session uuid: {:?}", self.session_id);

        Ok(())
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT>
    {
        println!("[profiler] Profiler successfully attached!");
        detach_after_duration::<AllocationByClassProfiler>(&self, 10);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        println!("[profiler] Profiler successfully detached!");

        let session = Session::create_session(self.session_id, AllocationByClassProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

        report.write_line(format!("# Allocations Report"));
        report.write_line(format!("## Total Collections"));
        report.write_line(format!("**Total Collections**: {}", self.collections.load(Ordering::Relaxed)));
        report.write_line(format!("## Allocations by Class"));

        use itertools::Itertools;

        report.write_line(format!("**Total Types**: {}", self.allocations_by_class.len()));

        for allocations_for_class in self.allocations_by_class.iter().sorted_by_key(|x| -x.value().load(Ordering::Relaxed)) {
            report.write_line(format!("- {}: {}", allocations_for_class.key(), allocations_for_class.value().load(Ordering::Relaxed)));
        }

        println!("[profiler] Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for AllocationByClassProfiler {}
impl CorProfilerCallback5 for AllocationByClassProfiler {}
impl CorProfilerCallback6 for AllocationByClassProfiler {}
impl CorProfilerCallback7 for AllocationByClassProfiler {}
impl CorProfilerCallback8 for AllocationByClassProfiler {}
impl CorProfilerCallback9 for AllocationByClassProfiler {}