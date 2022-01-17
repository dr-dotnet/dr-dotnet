use dashmap::DashMap;
use profiling_api::*;
use uuid::Uuid;
use std::sync::atomic::{AtomicIsize, Ordering};

use crate::report::*;
use crate::profilers::*;

pub struct MemoryLeakProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    surviving_references: DashMap<String, AtomicIsize>,
    collections: AtomicIsize,
}

impl Profiler for MemoryLeakProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E83").unwrap(),
            name: "Memory Leak Sniffer [WIP]".to_owned(),
            description: "Sniff sniff... oh look ma a leak".to_owned(),
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl Clone for MemoryLeakProfiler {
    fn clone(&self) -> Self { 
        MemoryLeakProfiler {
            profiler_info: self.profiler_info.clone(),
            session_id: self.session_id.clone(),
            surviving_references: DashMap::new(),
            collections: AtomicIsize::new(0)
        }
    }
}

impl ClrProfiler for MemoryLeakProfiler {
    fn new() -> MemoryLeakProfiler {
        MemoryLeakProfiler {
            profiler_info: None,
            session_id: Uuid::default(),
            surviving_references: DashMap::new(),
            collections: AtomicIsize::new(0)
        }
    }
}

impl CorProfilerCallback for MemoryLeakProfiler {}

impl CorProfilerCallback2 for MemoryLeakProfiler
{
    fn surviving_references(&mut self, object_id_range_start: &[ffi::ObjectID], object_id_range_length: &[u32]) -> Result<(), ffi::HRESULT>
    {
        println!("SURVIVING REFERENCES!");

        for i in 0..object_id_range_start.len()
        {
            let pinfo = self.profiler_info();
            let name = 
            match pinfo.get_class_from_object(object_id_range_start[i]) {
                Ok(class_id) => 
                match pinfo.get_class_id_info(class_id) {
                    Ok(class_info) => extensions::get_type_name(pinfo, class_info.module_id, class_info.token),
                    _ => "unknown2".to_owned()
                },
                _ => "unknown1".to_owned()
            };
    
            let key = name;
            match self.surviving_references.get_mut(&key) {
                Some(pair) => { pair.value().fetch_add(1, Ordering::Relaxed); },
                None => { self.surviving_references.insert(key, AtomicIsize::new(1)); },
            }
        }

        Ok(())
    }

    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT>
    {
        self.collections.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }
}

impl CorProfilerCallback3 for MemoryLeakProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        println!("[profiler] Initialize with attach");
        self.profiler_info = Some(profiler_info);

        match self.profiler_info().set_event_mask_2(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC, ffi::COR_PRF_HIGH_MONITOR::COR_PRF_HIGH_MONITOR_NONE) {
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
        detach_after_duration::<MemoryLeakProfiler>(&self, 10);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        println!("[profiler] Profiler successfully detached!");

        let session = Session::create_session(self.session_id, MemoryLeakProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

        report.write_line(format!("# Memory Leak Report"));
        report.write_line(format!("## Total Collections"));
        report.write_line(format!("**Total Collections**: {}", self.collections.load(Ordering::Relaxed)));
        report.write_line(format!("## Surviving References by Class"));

        use itertools::Itertools;

        for surviving_reference in self.surviving_references.iter().sorted_by_key(|x| -x.value().load(Ordering::Relaxed)) {
            report.write_line(format!("- {}: {}", surviving_reference.key(), surviving_reference.value().load(Ordering::Relaxed)));
        }

        println!("[profiler] Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for MemoryLeakProfiler
{
    // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-survivingreferences2-method
    fn surviving_references_2(&mut self, object_id_range_start: &[ffi::ObjectID], c_object_id_range_length: &[usize]) -> Result<(), ffi::HRESULT>
    {
        println!("SURVIVING REFERENCES 2!");

        for i in 0..object_id_range_start.len()
        {
            let pinfo = self.profiler_info();
            let name = match pinfo.get_class_from_object(object_id_range_start[i]) {
                Ok(class_id) =>
                match pinfo.get_class_id_info(class_id) {
                    Ok(class_info) => extensions::get_type_name(pinfo, class_info.module_id, class_info.token),
                    _ => "unknown2".to_owned()
                },
                _ => "unknown1".to_owned()
            };
    
            let key = name;
            let val = c_object_id_range_length[i] as isize;
            match self.surviving_references.get_mut(&key) {
                Some(pair) => { pair.value().fetch_add(val, Ordering::Relaxed); },
                None => { self.surviving_references.insert(key, AtomicIsize::new(val)); },
            }
        }

        Ok(())
    }
}

//impl CorProfilerCallback3 for MemoryLeakProfiler {}
impl CorProfilerCallback5 for MemoryLeakProfiler {}
impl CorProfilerCallback6 for MemoryLeakProfiler {}
impl CorProfilerCallback7 for MemoryLeakProfiler {}
impl CorProfilerCallback8 for MemoryLeakProfiler {}
impl CorProfilerCallback9 for MemoryLeakProfiler {}