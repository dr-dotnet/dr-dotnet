use std::sync::{ Arc, Mutex };
use std::sync::atomic::{ Ordering, AtomicBool, AtomicIsize };
use itertools::Itertools;

use crate::api::*;
use crate::api::ffi::{ FunctionID, HRESULT };
use crate::macros::*;
use crate::profilers::*;
use crate::session::Report;

#[derive(Default)]
pub struct CpuHotpathProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    merged_stack: Arc<Mutex<MergedStack>>
}

#[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
enum StackFrameType {
    #[default]
    Managed,
    Native
}

#[derive(Default, Clone, Debug)]
struct StackFrame {
    display: Option<String>,
    kind: StackFrameType,
    fct_id: FunctionID,
}

#[derive(Default, Clone, Debug)]
struct MergedStack {
    samples: u64,
    stacks: Vec<MergedStack>,
    frame: StackFrame
}

impl StackFrame {
    fn format(frame: &StackFrame, clr: &ClrProfilerInfo) -> String {
        match frame.kind {
            StackFrameType::Native => "unmanaged".to_string(),
            StackFrameType::Managed =>  unsafe { clr.get_full_method_name(frame.fct_id) }
        }
    }
}

impl MergedStack {

    pub fn increment(&mut self) {
        self.samples += 1;
    }

    pub fn add_stack(&mut self, clr: &ClrProfilerInfo, stack_trace: Vec<StackFrame>, index: Option<usize>) {
        self.samples += 1;

        let mut index = index.unwrap_or(0);
        let first_frame = &stack_trace[index];

        let mut merged_stack = self.stacks.iter_mut()
            .find(|s| s.frame.fct_id == first_frame.fct_id);

        if merged_stack.is_none() {
            self.stacks.push(MergedStack::new(&first_frame, clr));
            merged_stack = self.stacks.last_mut();
        }

        let merged_stack = merged_stack.unwrap();
        if index == stack_trace.len() -1 {
            merged_stack.increment();
        } else {
            merged_stack.add_stack(clr, stack_trace, Some(index + 1));
        }
    }

    pub fn new(frame: &StackFrame, clr: &ClrProfilerInfo) -> Self {
        MergedStack {
            samples: 0,
            stacks: Vec::new(),
            frame: StackFrame {
                kind: frame.kind.clone(),
                fct_id: frame.fct_id,
                display: Some(StackFrame::format(frame, clr))
            }
        }
    }

    fn write_html(&self, report: &mut Report, total_samples: u64, is_same: bool) {

        if self.stacks.is_empty() {
            if !is_same {
                report.write(format!("\n</details>\n"));
                report.write(self.render_as_html_summary(total_samples));
                report.write(format!("\n<details>\n\t"));
            } else {
                report.write(self.render_as_html_li(total_samples));
            }
            return;
        }

        if !is_same {
            report.write(format!("\n</details>\n"));
        }

        for next_stack in self.stacks.iter()
            .sorted_by(|a, b| Ord::cmp(&a.samples, &b.samples)) {

            let mut has_same_alignment = next_stack.samples == self.samples;

            if has_same_alignment {
                // Check that the next stack  of next_stack has also the same alignment
                let next_next_stack = next_stack.stacks.iter()
                    .sorted_by(|a, b| Ord::cmp(&a.samples, &b.samples))
                    .next();

                if let Some (n) = next_next_stack{
                    has_same_alignment = n.samples == self.samples;
                }
            }

            if has_same_alignment {
                report.write(format!("\n</ul>\n"));
            }

            next_stack.write_html(report, total_samples, has_same_alignment);

            if has_same_alignment {
                report.write(format!("\n<ul>\n"));
            }
        }

        if is_same {
            report.write(self.render_as_html_li(total_samples));
        } else {
            report.write(self.render_as_html_summary(total_samples));
            report.write(format!("\n<details>\n\t"));
        }
    }

    pub fn render_as_html_summary(&self, total_samples: u64) -> String {
        let frame = self.frame.display.as_ref().unwrap();
        let thread_count = format!("{:.2} %", 100f64 * self.samples as f64 / total_samples as f64);
        format!("<summary><span>{thread_count}</span>{frame}</summary>")
    }

    pub fn render_as_html_li(&self, total_samples: u64) -> String {
        let frame = self.frame.display.as_ref().unwrap();
        let thread_count = format!("{:.2} %", 100f64 * self.samples as f64 / total_samples as f64);
        format!("<li><span>{thread_count}</span>{frame}</li>")
    }
}

impl Profiler for CpuHotpathProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "805A308B-061C-47F3-9B30-A485B2056E71".to_owned(),
            name: "CPU Hotpath Profiler".to_owned(),
            description: "Lists CPU hotpaths.".to_owned(),
            is_released: true,
            parameters: vec![
                ProfilerParameter { 
                    name: "Duration".to_owned(),
                    key: "duration_seconds".to_owned(),
                    description: "The profiling duration in seconds".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "10".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter { 
                    name: "Time Interval".to_owned(),
                    key: "time_interval_ms".to_owned(),
                    description: "Time interval between two samples in milliseconds".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "40".to_owned(),
                    ..std::default::Default::default()
                }
            ],
            ..std::default::Default::default()
        }
    }
}

impl CpuHotpathProfiler {

    fn build_callstacks(profiler_info: ClrProfilerInfo, mut merged_stack: std::sync::MutexGuard<MergedStack>)
    {
        info!("Starts building callstacks");
        let pinfo = profiler_info.clone();

        for managed_thread_id in pinfo.enum_threads().unwrap() {

            let method_ids = Vec::<ffi::FunctionID>::new();

            // We must pass this data as a pointer for callback to mutate it with actual method ids from stack walking
            let method_ids_ptr_c = &method_ids as *const Vec<ffi::FunctionID> as *mut std::ffi::c_void;

            let _ =  pinfo.do_stack_snapshot(
                managed_thread_id,
                crate::utils::stack_snapshot_callback,
                ffi::COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_DEFAULT,
                method_ids_ptr_c,
                std::ptr::null(), 0);

            let mut stack_trace = Vec::<StackFrame>::new();

            for method_id in method_ids.iter().rev() {
                let frame = StackFrame {
                    kind: if *method_id == 0 { StackFrameType::Native } else { StackFrameType::Managed },
                    fct_id: *method_id,
                    display: None
                };
                //TODO: handle correctly native frame, for now we just ignore them
                if frame.kind == StackFrameType::Native { continue }
                stack_trace.push(frame);
            }

            if stack_trace.is_empty() { continue }

            merged_stack.add_stack(&profiler_info, stack_trace, None);
        }
    }
}

impl CorProfilerCallback for CpuHotpathProfiler {}

impl CorProfilerCallback2 for CpuHotpathProfiler {}

impl CorProfilerCallback3 for CpuHotpathProfiler {
    fn initialize_for_attach(&mut self, profiler_info: ClrProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT> {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT, None, profiler_info, client_data, client_data_length, None)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        let clr = self.clr().clone();
        let merged_stack = self.merged_stack.clone();

        let time_interval_ms = self.session_info().get_parameter::<u64>("time_interval_ms").unwrap();
        let duration_seconds = self.session_info().get_parameter::<u64>("duration_seconds").unwrap();

        std::thread::spawn(move || {
            let iterations = 1000 * duration_seconds / time_interval_ms;
            for i in 0..iterations {
                std::thread::sleep(std::time::Duration::from_millis(time_interval_ms));

                // https://github.com/dotnet/runtime/issues/37586#issuecomment-641114483
                if clr.suspend_runtime().is_ok() {

                    let k = merged_stack.lock().unwrap();
                    CpuHotpathProfiler::build_callstacks(clr.clone(), k);

                    if clr.resume_runtime().is_err() {
                        error!("Can't resume runtime!");
                    }
                } else {
                    error!("Can't suspend runtime!");
                }
            }

            clr.detach_now()
        });

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), HRESULT> {

        let mut report_html = self.session_info.create_report("stacks.html".to_owned());

        let merged_stack = self.merged_stack.lock().unwrap();

        for stack in merged_stack.stacks.iter()
            .sorted_by(|a, b| Ord::cmp(&a.samples, &b.samples)) {

            stack.write_html(&mut report_html, merged_stack.samples, false);
        }
    
        report_html.write_line(format!("<h3>{} samples <small class=\"text-muted\">with {} roots</small></h3>", merged_stack.samples, merged_stack.stacks.len()));

        match report_html.reverse_lines() {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to reverse lines of html report: {}", e)
            }
        };

        Ok(())
    }
}

impl CorProfilerCallback4 for CpuHotpathProfiler {}
impl CorProfilerCallback5 for CpuHotpathProfiler {}
impl CorProfilerCallback6 for CpuHotpathProfiler {}
impl CorProfilerCallback7 for CpuHotpathProfiler {}
impl CorProfilerCallback8 for CpuHotpathProfiler {}
impl CorProfilerCallback9 for CpuHotpathProfiler {}