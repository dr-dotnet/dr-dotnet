use profiling_api::*;
use uuid::Uuid;
use std::time::{Instant, Duration};

use crate::report::*;
use crate::profilers::*;

pub struct GCPause {
    start: Instant,
    end: Instant,
}

pub struct GCPauseProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    gc_pauses: Vec<GCPause>,
    last_start: Instant,
    profiling_start: Instant,
    profiling_end: Instant,
}

impl GCPauseProfiler {
    fn get_durations(&self, interval: Duration) -> Vec<Duration> {
        let mut current = self.profiling_start;
        let mut last_start = self.gc_pauses[0].start;
        let mut i: usize = 0;
        let mut durations = Vec::new();
        'outer: while current < self.profiling_end {
            current = current.checked_add(interval).unwrap();
            let mut duration= Duration::ZERO;
            while last_start < current {
                if self.gc_pauses[i].end < current {
                    duration += self.gc_pauses[i].end - last_start;
                    i += 1;
                    if  i >= self.gc_pauses.len() {
                        break 'outer
                    }
                    last_start = self.gc_pauses[i].start;
                }
                else
                {
                    duration += current - last_start;
                    last_start = current;
                }
            }
            durations.push(duration);
        }

        durations.sort_by(|a, b| a.partial_cmp(b).unwrap());

        return durations;
    }
}

impl Profiler for GCPauseProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E85").unwrap(),
            name: "GC Pause Profiler".to_owned(),
            description: "Measures the impact of GC pauses on response time".to_owned(),
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl Clone for GCPauseProfiler {
    fn clone(&self) -> Self { 
        GCPauseProfiler {
            profiler_info: self.profiler_info.clone(),
            session_id: self.session_id.clone(),
            gc_pauses: Vec::new(),
            last_start: Instant::now(),
            profiling_start: Instant::now(),
            profiling_end: Instant::now(),
        }
    }
}

impl ClrProfiler for GCPauseProfiler {
    fn new() -> GCPauseProfiler {
        GCPauseProfiler {
            profiler_info: None,
            session_id: Uuid::default(),
            gc_pauses: Vec::new(),
            last_start: Instant::now(),
            profiling_start: Instant::now(),
            profiling_end: Instant::now(),
        }
    }
}

impl CorProfilerCallback for GCPauseProfiler {

    fn runtime_suspend_started(&mut self, suspend_reason: ffi::COR_PRF_SUSPEND_REASON) -> Result<(), ffi::HRESULT> {
        self.last_start = Instant::now();
        Ok(())
    }

    fn runtime_resume_started(&mut self) -> Result<(), ffi::HRESULT> {
        let current = Instant::now();
        let gc_pause = GCPause { start: self.last_start, end: current };
        self.gc_pauses.push(gc_pause);
        Ok(())
    }
}

impl CorProfilerCallback2 for GCPauseProfiler {}

impl CorProfilerCallback3 for GCPauseProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        self.profiler_info = Some(profiler_info);

        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_SUSPENDS) {
            Ok(_) => (),
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
        self.profiling_start = Instant::now();
        detach_after_duration::<GCPauseProfiler>(&self, 20);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        self.profiling_end = Instant::now();

        let session = Session::get_session(self.session_id, GCPauseProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

        report.write_line(format!("# GC Pauses Report"));
        report.write_line(format!("## Time Spent Quantiles"));

        let total_time = self.profiling_end - self.profiling_start;
        let mut total_suspended_time = Duration::ZERO;
        for pause in self.gc_pauses.iter() {
            total_suspended_time += pause.end - pause.start;
        }
        let percentage_of_time_suspended = 100f64 * (total_suspended_time.as_secs_f64() / total_time.as_secs_f64());

        report.write_line(format!("- Percentage of time suspended: {}%", percentage_of_time_suspended));

        // Todo: make a table

        let durations_1000ms = self.get_durations(Duration::from_millis(1000));
        report.write_line(format!("- GC part on 99p over 1s delta: {}μs", durations_1000ms[(0.99f64 * (durations_1000ms.len() as f64)).floor() as usize].as_micros()));
        report.write_line(format!("- GC part on 95p over 1s delta: {}μs", durations_1000ms[(0.95f64 * (durations_1000ms.len() as f64)).floor() as usize].as_micros()));
        report.write_line(format!("- GC part on 50p over 1s delta: {}μs", durations_1000ms[(0.50f64 * (durations_1000ms.len() as f64)).floor() as usize].as_micros()));

        let durations_400ms = self.get_durations(Duration::from_millis(400));
        report.write_line(format!("- GC part on 99p over 400ms delta: {}μs", durations_400ms[(0.99f64 * (durations_400ms.len() as f64)).floor() as usize].as_micros()));
        report.write_line(format!("- GC part on 95p over 400ms delta: {}μs", durations_400ms[(0.95f64 * (durations_400ms.len() as f64)).floor() as usize].as_micros()));
        report.write_line(format!("- GC part on 50p over 400ms delta: {}μs", durations_400ms[(0.50f64 * (durations_400ms.len() as f64)).floor() as usize].as_micros()));

        let durations_50ms = self.get_durations(Duration::from_millis(50));
        report.write_line(format!("- GC part on 99p over 50ms delta: {}μs", durations_50ms[(0.99f64 * (durations_50ms.len() as f64)).floor() as usize].as_micros()));
        report.write_line(format!("- GC part on 95p over 50ms delta: {}μs", durations_50ms[(0.95f64 * (durations_50ms.len() as f64)).floor() as usize].as_micros()));
        report.write_line(format!("- GC part on 50p over 50ms delta: {}μs", durations_50ms[(0.50f64 * (durations_50ms.len() as f64)).floor() as usize].as_micros()));

        // let latency = durations_400ms
        //      .iter()
        //      .map(|&w| 100f64 * w.as_secs_f64() / Duration::from_millis(400).as_secs_f64());

        // for l in latency {
        //     report.write_line(format!("- {}%", l))
        // }

        info!("Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for GCPauseProfiler {}
impl CorProfilerCallback5 for GCPauseProfiler {}
impl CorProfilerCallback6 for GCPauseProfiler {}
impl CorProfilerCallback7 for GCPauseProfiler {}
impl CorProfilerCallback8 for GCPauseProfiler {}
impl CorProfilerCallback9 for GCPauseProfiler {}