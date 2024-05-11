use dashmap::DashMap;
use std::sync::atomic::{AtomicIsize, Ordering};

use crate::api::*;
use crate::macros::*;
use crate::profilers::*;
use crate::utils::NameResolver;

#[derive(Default)]
pub struct ExceptionsProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    exceptions: DashMap<String, AtomicIsize>,
}

impl Profiler for ExceptionsProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "805A308B-061C-47F3-9B30-F785C3186E82".to_owned(),
            name: "Count thrown exceptions by type".to_owned(),
            description: "Lists occuring exceptions by importance.\nHandled exceptions are also listed.".to_owned(),
            parameters: vec![ProfilerParameter {
                name: "Duration".to_owned(),
                key: "duration".to_owned(),
                description: "The profiling duration in seconds".to_owned(),
                type_: ParameterType::INT.into(),
                value: "10".to_owned(),
                ..std::default::Default::default()
            }],
            ..std::default::Default::default()
        };
    }
}

impl CorProfilerCallback for ExceptionsProfiler {
    fn exception_thrown(&mut self, thrown_object_id: ffi::ObjectID) -> Result<(), ffi::HRESULT> {
        let clr = self.clr();
        let name = match clr.get_class_from_object(thrown_object_id) {
            Ok(class_id) => clr.get_class_name(class_id),
            _ => "unknown".to_owned(),
        };

        let key = name;
        match self.exceptions.get_mut(&key) {
            Some(pair) => {
                pair.value().fetch_add(1, Ordering::Relaxed);
            }
            None => {
                self.exceptions.insert(key, AtomicIsize::new(1));
            }
        }

        Ok(())
    }
}

impl CorProfilerCallback2 for ExceptionsProfiler {}

impl CorProfilerCallback3 for ExceptionsProfiler {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ClrProfilerInfo,
        client_data: *const std::os::raw::c_void,
        client_data_length: u32,
    ) -> Result<(), ffi::HRESULT> {
        self.init(
            ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_EXCEPTIONS,
            None,
            profiler_info,
            client_data,
            client_data_length,
        )
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        let duration_seconds = self.session_info().get_parameter::<u64>("duration").unwrap();

        detach_after_duration::<ExceptionsProfiler>(&self, duration_seconds);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT> {
        let mut report = self.session_info().create_report("summary.md".to_owned());

        report.write_line(format!("# Exceptions Report"));
        report.write_line(format!("## Exceptions by Occurrences"));

        use itertools::Itertools;

        for exception in self.exceptions.iter().sorted_by_key(|x| -x.value().load(Ordering::Relaxed)) {
            report.write_line(format!("- {}: {}", exception.key(), exception.value().load(Ordering::Relaxed)));
        }

        self.session_info.finish();

        info!("Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for ExceptionsProfiler {}
impl CorProfilerCallback5 for ExceptionsProfiler {}
impl CorProfilerCallback6 for ExceptionsProfiler {}
impl CorProfilerCallback7 for ExceptionsProfiler {}
impl CorProfilerCallback8 for ExceptionsProfiler {}
impl CorProfilerCallback9 for ExceptionsProfiler {}
