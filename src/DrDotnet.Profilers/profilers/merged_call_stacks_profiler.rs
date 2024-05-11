use itertools::Itertools;
use std::cmp::min;
use std::sync::{Arc, Mutex};

use crate::api::ffi::{FunctionID, ThreadID};
use crate::api::*;
use crate::macros::*;
use crate::profilers::*;
use crate::session::Report;
use crate::utils::{NameResolver, StackSnapshotCallbackReceiver};

const PADDING: usize = 5;
/// if < 0 will print all thread ids
const NB_THREAD_IDS_TO_PRINT: usize = 4;

impl Profiler for MergedCallStacksProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "9404d16c-b49e-11ed-afa1-0242ac120002".to_owned(),
            name: "List merged call stacks".to_owned(),
            description: "Lists threads call stacks merged by stack frame.".to_owned(),
            ..std::default::Default::default()
        };
    }
}

#[derive(Default)]
pub struct MergedCallStacksProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    merged_stack: Arc<Mutex<MergedStack>>,
}

#[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
enum StackFrameType {
    #[default]
    Managed,
    Native,
}

#[derive(Default, Clone, Debug)]
struct StackFrame {
    display: Option<String>,
    kind: StackFrameType,
    fct_id: FunctionID,
}

#[derive(Default, Clone, Debug)]
struct MergedStack {
    thread_ids: Vec<ThreadID>,
    stacks: Vec<MergedStack>,
    frame: StackFrame,
}

impl StackFrame {
    fn format(frame: &StackFrame, clr: &ClrProfilerInfo) -> String {
        match frame.kind {
            StackFrameType::Native => "unmanaged".to_string(),
            StackFrameType::Managed => clr.clone().get_full_method_name(frame.fct_id),
        }
    }
}

impl MergedStack {
    pub fn push_thread_id(&mut self, thread_id: ThreadID) {
        self.thread_ids.push(thread_id);
    }

    pub fn add_stack(&mut self, clr: &ClrProfilerInfo, thread_id: ThreadID, stack_trace: Vec<StackFrame>, index: Option<usize>) {
        self.thread_ids.push(thread_id);

        let index = index.unwrap_or(0);
        let first_frame = &stack_trace[index];

        let mut merged_stack = self.stacks.iter_mut().find(|s| s.frame.fct_id == first_frame.fct_id);

        if merged_stack.is_none() {
            self.stacks.push(MergedStack::new(&first_frame, clr));
            merged_stack = self.stacks.last_mut();
        }

        let merged_stack = merged_stack.unwrap();
        if index == stack_trace.len() - 1 {
            merged_stack.push_thread_id(thread_id);
        } else {
            merged_stack.add_stack(clr, thread_id, stack_trace, Some(index + 1));
        }
    }

    pub fn new(frame: &StackFrame, clr: &ClrProfilerInfo) -> Self {
        MergedStack {
            thread_ids: Vec::new(),
            stacks: Vec::new(),
            frame: StackFrame {
                kind: frame.kind.clone(),
                fct_id: frame.fct_id,
                display: Some(StackFrame::format(frame, clr)),
            },
        }
    }

    pub fn write_to(&self, report: &mut Report) {
        self.write_stack(report, 0);
    }

    fn write_html(&self, report: &mut Report, is_same: bool) {
        if self.stacks.is_empty() {
            if !is_same {
                report.write(format!("\n</details>\n"));
                report.write(self.render_as_html_summary());
                report.write(format!("\n<details>\n\t"));
            } else {
                report.write(self.render_as_html_li());
            }
            return;
        }

        if !is_same {
            report.write(format!("\n</details>\n"));
        }

        for next_stack in self.stacks.iter().sorted_by(|a, b| Ord::cmp(&a.thread_ids.len(), &b.thread_ids.len())) {
            let mut has_same_alignment = next_stack.thread_ids.len() == self.thread_ids.len();

            if has_same_alignment {
                // Check that the next stack  of next_stack has also the same alignment
                let next_next_stack = next_stack
                    .stacks
                    .iter()
                    .sorted_by(|a, b| Ord::cmp(&a.thread_ids.len(), &b.thread_ids.len()))
                    .next();

                if let Some(n) = next_next_stack {
                    has_same_alignment = n.thread_ids.len() == self.thread_ids.len();
                }
            }

            if has_same_alignment {
                report.write(format!("\n</ul>\n"));
            }

            next_stack.write_html(report, has_same_alignment);

            if has_same_alignment {
                report.write(format!("\n<ul>\n"));
            }
        }

        if is_same {
            report.write(self.render_as_html_li());
        } else {
            report.write(self.render_as_html_summary());
            report.write(format!("\n<details>\n\t"));
        }
    }

    fn write_stack(&self, report: &mut Report, increment: usize) {
        let alignment = str::repeat(" ", PADDING * increment);
        let new_line = format!("\r\n{alignment}");

        let thread_count = format!("{:>PADDING$} ", self.thread_ids.len());
        let frame = self.frame.display.as_ref().unwrap();

        if self.stacks.is_empty() {
            let thread_ids = format!(" ~~~~ {}", self.format_thread_ids());
            report.write(new_line.as_str());
            report.write(thread_ids);
            report.write(new_line.as_str());
            report.write(thread_count);
            report.write(frame);
            return;
        }

        for next_stack in self.stacks.iter().sorted_by(|a, b| Ord::cmp(&b.thread_ids.len(), &a.thread_ids.len())) {
            let has_same_alignment = next_stack.thread_ids.len() == self.thread_ids.len();
            next_stack.write_stack(report, if has_same_alignment { increment } else { increment + 1 });
        }

        report.write(new_line.as_str());
        report.write(thread_count);
        report.write(frame);
    }

    fn format_thread_ids(&self) -> String {
        let count = self.thread_ids.len();
        let limit = min(count, NB_THREAD_IDS_TO_PRINT);

        let mut result = self
            .thread_ids
            .get(..limit)
            .unwrap_or_default()
            .iter()
            .map(|k| format!("{k}"))
            .collect::<Vec<String>>()
            .join(",");

        if count > limit {
            result += "...";
        }
        result
    }

    pub fn render_as_html_summary(&self) -> String {
        let frame = self.frame.display.as_ref().unwrap();
        let thread_count = format!("{}", self.thread_ids.len());
        format!("<summary><span>{thread_count}</span>{frame}</summary>")
    }

    pub fn render_as_html_li(&self) -> String {
        let frame = self.frame.display.as_ref().unwrap();
        let thread_count = format!("{}", self.thread_ids.len());
        format!("<li><span>{thread_count}</span>{frame}</li>")
    }
}

#[derive(Default)]
pub struct MergedCallstacksStackSnapshotCallbackReceiver {
    method_ids: Vec<ffi::FunctionID>,
}

impl StackSnapshotCallbackReceiver for MergedCallstacksStackSnapshotCallbackReceiver {
    type AssociatedType = Self;

    fn callback(&mut self, method_id: FunctionID, ip: usize, _: usize, _: &[u8]) {
        self.method_ids.push(method_id);
    }
}

impl MergedCallStacksProfiler {
    fn build_callstacks(profiler_info: ClrProfilerInfo, mut merged_stack: std::sync::MutexGuard<MergedStack>) {
        info!("Starts building callstacks");
        let pinfo = profiler_info.clone();

        for managed_thread_id in pinfo.enum_threads().unwrap() {
            let mut stack_snapshot_receiver = MergedCallstacksStackSnapshotCallbackReceiver::default();

            stack_snapshot_receiver.do_stack_snapshot(pinfo.clone(), managed_thread_id, false);

            let mut stack_trace = Vec::<StackFrame>::new();

            for method_id in stack_snapshot_receiver.method_ids.iter().rev() {
                let frame = StackFrame {
                    kind: if *method_id == 0 { StackFrameType::Native } else { StackFrameType::Managed },
                    fct_id: *method_id,
                    display: None,
                };
                //TODO: handle correctly native frame, for now we just ignore them
                if frame.kind == StackFrameType::Native {
                    continue;
                }
                stack_trace.push(frame);
            }

            if stack_trace.is_empty() {
                continue;
            }

            merged_stack.add_stack(&profiler_info, managed_thread_id, stack_trace, None);
        }
    }
}

impl CorProfilerCallback for MergedCallStacksProfiler {}

impl CorProfilerCallback2 for MergedCallStacksProfiler {}

impl CorProfilerCallback3 for MergedCallStacksProfiler {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ClrProfilerInfo,
        client_data: *const std::os::raw::c_void,
        client_data_length: u32,
    ) -> Result<(), ffi::HRESULT> {
        self.init(
            ffi::COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT,
            None,
            profiler_info,
            client_data,
            client_data_length,
        )
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        let profiler_info = self.clr().clone();
        let merged_stack = self.merged_stack.clone();

        let thread_handle = std::thread::spawn(move || {
            // https://github.com/dotnet/runtime/issues/37586#issuecomment-641114483
            if profiler_info.suspend_runtime().is_ok() {
                let k = merged_stack.lock().unwrap();
                MergedCallStacksProfiler::build_callstacks(profiler_info.clone(), k);

                if profiler_info.resume_runtime().is_err() {
                    error!("Can't resume runtime!");
                }
            } else {
                error!("Can't suspend runtime!");
            }
        });

        if thread_handle.join().is_err() {
            error!("Can't wait for the thread to finish!");
        }

        self.clr().request_profiler_detach(3000)
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT> {
        let mut report = self.session_info.create_report("pstacks.md".to_owned());
        report.write_line(format!("# Merged Callstacks"));

        let mut report_html = self.session_info.create_report("collapsible_pstacks.html".to_owned());

        let merged_stack = self.merged_stack.lock().unwrap();

        for stack in merged_stack.stacks.iter().sorted_by(|a, b| Ord::cmp(&a.thread_ids.len(), &b.thread_ids.len())) {
            stack.write_to(&mut report);
            report.write(format!("\n\n{}", str::repeat("_", 50)));

            stack.write_html(&mut report_html, false);
        }
        report.write_line(format!(
            "\n==> {} threads with {} roots",
            merged_stack.thread_ids.len(),
            merged_stack.stacks.len()
        ));
        report_html.write_line(format!(
            "<h3>{} threads <small class=\"text-muted\">with {} roots</small></h3>",
            merged_stack.thread_ids.len(),
            merged_stack.stacks.len()
        ));

        match report_html.reverse_lines() {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to reverse lines of html report: {}", e)
            }
        };

        self.session_info.finish();

        Ok(())
    }
}

impl CorProfilerCallback4 for MergedCallStacksProfiler {}
impl CorProfilerCallback5 for MergedCallStacksProfiler {}
impl CorProfilerCallback6 for MergedCallStacksProfiler {}
impl CorProfilerCallback7 for MergedCallStacksProfiler {}
impl CorProfilerCallback8 for MergedCallStacksProfiler {}
impl CorProfilerCallback9 for MergedCallStacksProfiler {}
