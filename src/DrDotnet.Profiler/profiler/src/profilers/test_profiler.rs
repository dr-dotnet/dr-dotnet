use profiling_api::*;
use uuid::Uuid;

use std::fs::File;
use std::io::{BufWriter, Write};

use crate::profilers::*;

pub struct TestProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    writer: BufWriter<File>,
}

impl Profiler for TestProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("705A308B-061C-47F3-9B30-F785C3186E82").unwrap(),
            name: "Test Profiler".to_owned(),
            description: "Profiler for unit test purpose.".to_owned(),
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl Clone for TestProfiler {
    fn clone(&self) -> Self { 

        let test_output_file = File::create("test_output.txt").unwrap();
        let mut writer = BufWriter::new(test_output_file);

        TestProfiler {
            profiler_info: self.profiler_info.clone(),
            session_id: self.session_id.clone(),
            writer: writer,
        }
    }
}

impl ClrProfiler for TestProfiler {
    fn new() -> TestProfiler {

        let test_output_file = File::create("test_output.txt").unwrap();
        let mut writer = BufWriter::new(test_output_file);

        TestProfiler {
            profiler_info: None,
            session_id: Uuid::default(),
            writer: writer,
        }
    }
}

impl CorProfilerCallback for TestProfiler
{
    fn exception_thrown(&mut self, thrown_object_id: ffi::ObjectID) -> Result<(), ffi::HRESULT>
    {
        self.writer.write(b"exception_thrown\n");
        Ok(())
    }
}

impl CorProfilerCallback2 for TestProfiler {}

impl CorProfilerCallback3 for TestProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        self.writer.write(b"initialize_for_attach\n");
        self.profiler_info = Some(profiler_info);
        self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_EXCEPTIONS /*| ffi::COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT*/)?;
        Ok(())
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT>
    {
        self.writer.write(b"profiler_attach_complete\n");
        detach_after_duration::<TestProfiler>(&self, 5);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        self.writer.write(b"profiler_detach_succeeded\n");
        Ok(())
    }
}

impl CorProfilerCallback4 for TestProfiler {}
impl CorProfilerCallback5 for TestProfiler {}
impl CorProfilerCallback6 for TestProfiler {}
impl CorProfilerCallback7 for TestProfiler {}
impl CorProfilerCallback8 for TestProfiler {}
impl CorProfilerCallback9 for TestProfiler {}