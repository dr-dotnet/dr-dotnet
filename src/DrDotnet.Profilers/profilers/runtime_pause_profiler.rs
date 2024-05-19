use crate::api::*;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use std::time::{Duration, Instant};

use crate::macros::*;
use crate::profilers::*;

#[derive(Clone)]
pub struct RuntimePause {
    time: DateTime<Utc>,
    start: Instant,
    end: Instant,
    reason: ffi::COR_PRF_SUSPEND_REASON,
    gc_reason: Option<ffi::COR_PRF_GC_REASON>,
    gc_gen: Option<i8>,
}

pub struct RuntimePauseProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    profiling_start: Instant,
    profiling_end: Instant,
    gc_pauses: Vec<RuntimePause>,
    current_pause: Option<RuntimePause>,
}

impl Default for RuntimePauseProfiler {
    fn default() -> RuntimePauseProfiler {
        RuntimePauseProfiler {
            clr_profiler_info: ClrProfilerInfo::default(),
            session_info: SessionInfo::default(),
            profiling_start: Instant::now(),
            profiling_end: Instant::now(),
            gc_pauses: Vec::new(),
            current_pause: None,
        }
    }
}

impl Profiler for RuntimePauseProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "805A308B-061C-47F3-9B30-F785C3186E85".to_owned(),
            name: "List runtime pauses".to_owned(),
            description: "Lists runtime pauses and their durations, such as blocking garbage collections.".to_owned(),
            ..std::default::Default::default()
        };
    }
}

impl CorProfilerCallback for RuntimePauseProfiler {
    fn runtime_suspend_started(&mut self, suspend_reason: ffi::COR_PRF_SUSPEND_REASON) -> Result<(), ffi::HRESULT> {
        self.current_pause = Some(RuntimePause {
            time: Utc::now(),
            start: Instant::now(),
            end: Instant::now(),
            reason: suspend_reason.clone(),
            gc_gen: None,
            gc_reason: None,
        });
        Ok(())
    }

    fn runtime_resume_started(&mut self) -> Result<(), ffi::HRESULT> {
        if let Some(mut current_pause) = self.current_pause.take() {
            current_pause.end = Instant::now();
            self.gc_pauses.push(current_pause);
        } else {
            error!("Runtime resume started but there is no current pause tracked");
        }
        Ok(())
    }
}

impl CorProfilerCallback2 for RuntimePauseProfiler {
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT> {
        if self.current_pause.is_some() {
            let mut current_pause = self.current_pause.clone().unwrap();
            current_pause.gc_reason = Some(reason);
            current_pause.gc_gen = Some(ClrProfilerInfo::get_gc_gen(&generation_collected));
            self.current_pause = Some(current_pause);
        } else {
            error!("Garbage collection started but there is no current pause tracked");
        }
        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), ffi::HRESULT> {
        Ok(())
    }
}

impl CorProfilerCallback3 for RuntimePauseProfiler {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ClrProfilerInfo,
        client_data: *const std::os::raw::c_void,
        client_data_length: u32,
    ) -> Result<(), ffi::HRESULT> {
        self.init(
            ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_SUSPENDS,
            Some(ffi::COR_PRF_HIGH_MONITOR::COR_PRF_HIGH_BASIC_GC),
            profiler_info,
            client_data,
            client_data_length,
        )
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        self.profiling_start = Instant::now();
        detach_after_duration::<RuntimePauseProfiler>(&self, 20);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT> {
        self.profiling_end = Instant::now();

        let mut report = self.session_info.create_report("summary.md".to_owned());

        report.write_line(format!("# Runtime Pauses Report"));
        report.write_line(format!("## General"));

        let total_time = self.profiling_end - self.profiling_start;
        let mut total_suspended_time = Duration::ZERO;
        for pause in self.gc_pauses.iter() {
            total_suspended_time += pause.end - pause.start;
        }
        let percentage_of_time_suspended = 100f64 * (total_suspended_time.as_secs_f64() / total_time.as_secs_f64());

        report.write_line(format!("- Number of pauses: {}", self.gc_pauses.len()));
        report.write_line(format!("- Percentage of time suspended: {}%", percentage_of_time_suspended));

        if self.gc_pauses.len() > 0 {
            let pauses_sorted = self.gc_pauses.iter().map(|pause| pause.end - pause.start).sorted().collect_vec();

            report.write_line(format!("- Longuest pause time: {}ms", pauses_sorted.last().unwrap().as_millis()));

            let avg_time_spent = self.gc_pauses.iter().map(|pause| pause.end - pause.start).sum::<Duration>() / self.gc_pauses.len() as u32;

            report.write_line(format!("- Average pause time: {}ms", avg_time_spent.as_millis()));

            report.write_line(format!("## Quantiles"));

            report.write_line(format!(
                "- 50p (median): {}ms",
                pauses_sorted[(0.50f64 * (pauses_sorted.len() as f64)).floor() as usize].as_millis()
            ));
            report.write_line(format!(
                "- 95p: {}ms",
                pauses_sorted[(0.95f64 * (pauses_sorted.len() as f64)).floor() as usize].as_millis()
            ));
            report.write_line(format!(
                "- 99p: {}ms",
                pauses_sorted[(0.99f64 * (pauses_sorted.len() as f64)).floor() as usize].as_millis()
            ));

            report.write_line(format!("## All Pauses"));
            report.new_line();
            report.write_line(format!("Iteration | Time (UTC) | Pause Reason | Duration (ms)"));
            report.write_line(format!("-: | -: | -: | -:"));

            let mut i = 1;
            for pause in self.gc_pauses.iter() {
                let reason: String = match &pause.gc_reason {
                    Some(gc_reason) => format!("{:?} ({:?} Gen {})", pause.reason, gc_reason, pause.gc_gen.unwrap()),
                    None => format!("kk{:?}", pause.reason),
                };
                report.write_line(format!("{} | {} | {} | {}", i, pause.time, reason, (pause.end - pause.start).as_millis()));
                i += 1;
            }
        }

        self.session_info.finish();

        info!("Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for RuntimePauseProfiler {}
impl CorProfilerCallback5 for RuntimePauseProfiler {}
impl CorProfilerCallback6 for RuntimePauseProfiler {}
impl CorProfilerCallback7 for RuntimePauseProfiler {}
impl CorProfilerCallback8 for RuntimePauseProfiler {}
impl CorProfilerCallback9 for RuntimePauseProfiler {}
