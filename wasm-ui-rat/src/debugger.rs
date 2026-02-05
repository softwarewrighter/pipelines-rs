//! Visual debugger component for record-at-a-time pipeline inspection.
//!
//! Steps through input records (not stages). Each step shows one record's
//! journey through all pipe points simultaneously. After all records,
//! flush traces are shown.

use pipelines_rs::RatDebugTrace;
use yew::prelude::*;

use crate::dsl::PipelineLine;

/// A watch placed at a pipe point between stages.
#[derive(Clone, PartialEq)]
pub struct Watch {
    /// Display label: "w1", "w2", etc.
    pub label: String,
    /// Which pipe point (output of this stage index).
    pub stage_index: usize,
}

/// Debugger state (stored in AppState).
#[derive(Clone, PartialEq)]
pub struct DebuggerState {
    /// Whether the debugger has run at least once.
    pub active: bool,
    /// Debug trace from RAT pipeline execution.
    pub trace: Option<RatDebugTrace>,
    /// Current step: 0..total_steps (record traces then flush traces).
    pub current_step: usize,
    /// Total steps: record_traces.len() + flush_traces.len().
    pub total_steps: usize,
    /// Ordered list of watches.
    pub watches: Vec<Watch>,
    /// Counter for generating watch labels.
    pub next_watch_id: usize,
    /// Total number of stages (not counting source).
    pub stage_count: usize,
    /// Pipeline output text.
    pub output_text: String,
    /// Input record count.
    pub input_count: usize,
    /// Output record count.
    pub output_count: usize,
    /// Pipeline lines for display.
    pub pipeline_lines: Vec<PipelineLine>,
    /// Error from pipeline execution.
    pub error: Option<String>,
}

impl Default for DebuggerState {
    fn default() -> Self {
        Self {
            active: false,
            trace: None,
            current_step: 0,
            total_steps: 0,
            watches: Vec::new(),
            next_watch_id: 1,
            stage_count: 0,
            output_text: String::new(),
            input_count: 0,
            output_count: 0,
            pipeline_lines: Vec::new(),
            error: None,
        }
    }
}

impl DebuggerState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a watch at the given pipe point.
    pub fn add_watch(&mut self, stage_index: usize) {
        let label = format!("w{}", self.next_watch_id);
        self.next_watch_id += 1;
        self.watches.push(Watch { label, stage_index });
    }

    /// Remove a watch by label.
    pub fn remove_watch(&mut self, label: &str) {
        self.watches.retain(|w| w.label != label);
    }

    /// Get watches at a specific pipe point.
    pub fn watches_at(&self, stage_index: usize) -> Vec<&Watch> {
        self.watches
            .iter()
            .filter(|w| w.stage_index == stage_index)
            .collect()
    }

    /// Number of record traces.
    fn record_count(&self) -> usize {
        self.trace
            .as_ref()
            .map(|t| t.record_traces.len())
            .unwrap_or(0)
    }

    /// Number of flush traces.
    fn flush_count(&self) -> usize {
        self.trace
            .as_ref()
            .map(|t| t.flush_traces.len())
            .unwrap_or(0)
    }

    /// Step counter label: "Record 2 of 8" or "Flush 1 of 2".
    ///
    /// `current_step` is 0 before any stepping; after one step it becomes 1
    /// and the display shows `trace[0]`. So the label number equals
    /// `current_step` (not `current_step + 1`).
    fn step_label(&self) -> String {
        if !self.active || self.total_steps == 0 || self.current_step == 0 {
            return String::new();
        }
        let rc = self.record_count();
        if self.current_step <= rc {
            format!("Record {} of {}", self.current_step, rc)
        } else {
            let flush_num = self.current_step - rc;
            let fc = self.flush_count();
            format!("Flush {} of {}", flush_num, fc)
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DebuggerProps {
    pub state: DebuggerState,
    pub on_run: Callback<()>,
    pub on_step: Callback<()>,
    pub on_run_all: Callback<()>,
    pub on_reset: Callback<()>,
    pub on_add_watch: Callback<usize>,
    pub on_remove_watch: Callback<String>,
}

/// Visual debugger panel component.
#[function_component(DebuggerPanel)]
pub fn debugger_panel(props: &DebuggerProps) -> Html {
    let state = &props.state;

    let on_run = {
        let cb = props.on_run.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    let on_step = {
        let cb = props.on_step.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    let on_run_all = {
        let cb = props.on_run_all.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    let on_reset = {
        let cb = props.on_reset.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    let step_label = state.step_label();

    html! {
        <div class="panel debugger-panel">
            <div class="panel-header">
                <h2>{"Visual Debugger"}</h2>
                <div class="debug-controls">
                    <button class="debug-btn" onclick={on_run} title="Run pipeline">
                        {"Run"}
                    </button>
                    <button class="debug-btn"
                        onclick={on_step}
                        disabled={!state.active || state.current_step >= state.total_steps}
                        title="Step to next record"
                    >
                        {"Step \u{25B6}"}
                    </button>
                    <button class="debug-btn"
                        onclick={on_run_all}
                        disabled={!state.active || state.current_step >= state.total_steps}
                        title="Run all remaining steps"
                    >
                        {"Run All"}
                    </button>
                    <button class="debug-btn"
                        onclick={on_reset}
                        disabled={!state.active}
                        title="Reset to step 0"
                    >
                        {"Reset"}
                    </button>
                    if !step_label.is_empty() {
                        <span class="step-counter">{step_label}</span>
                    }
                </div>
            </div>
            <div class="panel-content debugger-content">
                { render_error(state) }
                { render_stage_list(state, &props.on_add_watch) }
                { render_watch_list(state, &props.on_remove_watch) }
            </div>
        </div>
    }
}

fn render_error(state: &DebuggerState) -> Html {
    if let Some(err) = &state.error {
        html! {
            <div class="error">{err}</div>
        }
    } else {
        html! {}
    }
}

fn render_stage_list(state: &DebuggerState, on_add_watch: &Callback<usize>) -> Html {
    if !state.active {
        return html! {
            <div class="debugger-placeholder">
                <p>{"Click "}<strong>{"Run"}</strong>{" to load the pipeline, then "}<strong>{"Step"}</strong>{" through records."}</p>
                <p class="hint">{"Each step shows one record's journey through all stages."}</p>
            </div>
        };
    }

    let lines = &state.pipeline_lines;

    html! {
        <div class="stage-list">
            { for lines.iter().enumerate().map(|(i, line)| {
                let stage_idx = line.stage_index;

                html! {
                    <>
                        // Stage row
                        <div class={classes!("stage-line", stage_class(state, stage_idx))}>
                            <span class="stage-prefix">
                                { if stage_idx == 0 { "PIPE" } else { "|" } }
                            </span>
                            <span class="stage-text">{&line.text}</span>
                            <span class="stage-number">{format!("stage {stage_idx}")}</span>
                        </div>
                        // Pipe point between stages (not after last)
                        { if i < lines.len() - 1 {
                            render_pipe_point(state, stage_idx, on_add_watch)
                        } else {
                            html! {}
                        }}
                    </>
                }
            })}
        </div>
    }
}

/// Determine the CSS class for a stage row based on current step.
///
/// In RAT mode all stages are always "visible" since we show a record's
/// full journey. We highlight based on whether the current step has data
/// flowing through this stage.
fn stage_class(state: &DebuggerState, stage_idx: usize) -> &'static str {
    if state.current_step == 0 || state.current_step > state.total_steps {
        return "stage-pending";
    }

    let trace = match &state.trace {
        Some(t) => t,
        None => return "stage-pending",
    };

    let rc = trace.record_traces.len();
    let step = state.current_step - 1; // 0-based index into traces

    if step < rc {
        // Record trace step
        let rt = &trace.record_traces[step];
        // pipe_points[stage_idx] is input to stage_idx (output of stage_idx-1)
        // pipe_points[stage_idx+1] is output of stage_idx
        // Stage is "active" if it received input
        if stage_idx < rt.pipe_points.len() && !rt.pipe_points[stage_idx].is_empty() {
            "stage-completed"
        } else {
            "stage-pending"
        }
    } else {
        // Flush trace step
        let flush_idx = step - rc;
        if let Some(ft) = trace.flush_traces.get(flush_idx) {
            // Flush originates at ft.stage_index, propagates downstream
            // Stages at or after the originating stage are relevant
            if stage_idx >= ft.stage_index {
                let offset = stage_idx - ft.stage_index;
                if offset < ft.pipe_points.len() && !ft.pipe_points[offset].is_empty() {
                    "stage-completed"
                } else {
                    "stage-pending"
                }
            } else {
                "stage-pending"
            }
        } else {
            "stage-pending"
        }
    }
}

fn render_pipe_point(
    state: &DebuggerState,
    stage_index: usize,
    on_add_watch: &Callback<usize>,
) -> Html {
    let watches = state.watches_at(stage_index);

    let record_info = pipe_point_info(state, stage_index);

    let on_click = {
        let cb = on_add_watch.clone();
        let idx = stage_index;
        Callback::from(move |_: MouseEvent| cb.emit(idx))
    };

    let has_data = state.current_step > 0 && !record_info.starts_with('\u{00B7}');
    let pipe_class = if has_data {
        "pipe-point pipe-reached"
    } else {
        "pipe-point pipe-pending"
    };

    // \u{24E6} = ⓦ (circled w)
    html! {
        <div class={pipe_class} onclick={on_click} title="Click to add watch">
            <span class="pipe-watch-icon">{"\u{24E6}"}</span>
            { for watches.iter().map(|w| {
                html! {
                    <span class="watch-label">{&w.label}</span>
                }
            })}
            <span class="pipe-info">{record_info}</span>
        </div>
    }
}

/// Get the pipe point info text for the current step.
///
/// For a record step, shows what's at pipe_points[stage_index + 1]
/// (the output of stage_index, input to stage_index + 1).
/// For a flush step, shows what's at the appropriate offset from the
/// originating stage.
fn pipe_point_info(state: &DebuggerState, stage_index: usize) -> String {
    if state.current_step == 0 {
        return "\u{00B7}\u{00B7}\u{00B7}".to_string();
    }

    let trace = match &state.trace {
        Some(t) => t,
        None => return "\u{00B7}\u{00B7}\u{00B7}".to_string(),
    };

    let rc = trace.record_traces.len();
    let step = state.current_step - 1;

    // Pipe point between stage_index and stage_index+1 corresponds
    // to pipe_points[stage_index + 1] in a record trace
    let pp_index = stage_index + 1;

    if step < rc {
        let rt = &trace.record_traces[step];
        if pp_index < rt.pipe_points.len() {
            let records = &rt.pipe_points[pp_index];
            format_pipe_point_records(records)
        } else {
            "\u{00B7}\u{00B7}\u{00B7}".to_string()
        }
    } else {
        let flush_idx = step - rc;
        if let Some(ft) = trace.flush_traces.get(flush_idx) {
            // Flush pipe point: stage_index must be >= ft.stage_index
            if stage_index >= ft.stage_index {
                let offset = stage_index - ft.stage_index + 1;
                if offset < ft.pipe_points.len() {
                    let records = &ft.pipe_points[offset];
                    format_pipe_point_records(records)
                } else {
                    "\u{00B7}\u{00B7}\u{00B7}".to_string()
                }
            } else {
                // Above the flush origin
                "\u{2014}".to_string()
            }
        } else {
            "\u{00B7}\u{00B7}\u{00B7}".to_string()
        }
    }
}

/// Format records at a pipe point for display.
fn format_pipe_point_records(records: &[pipelines_rs::Record]) -> String {
    if records.is_empty() {
        "(filtered out)".to_string()
    } else if records.len() == 1 {
        let preview = records[0].as_str().trim_end();
        let truncated = if preview.len() > 30 {
            format!("{}...", &preview[..27])
        } else {
            preview.to_string()
        };
        format!("1 rec: {truncated}")
    } else {
        let preview = records[0].as_str().trim_end();
        let truncated = if preview.len() > 25 {
            format!("{}...", &preview[..22])
        } else {
            preview.to_string()
        };
        format!("{} recs: {truncated}", records.len())
    }
}

fn render_watch_list(state: &DebuggerState, on_remove_watch: &Callback<String>) -> Html {
    if !state.active {
        return html! {};
    }

    html! {
        <div class="watch-list">
            <h3 class="watch-list-header">{"Watches"}</h3>
            if state.watches.is_empty() {
                <p class="watch-hint">{"Click a pipe point to add a watch"}</p>
            } else {
                { for state.watches.iter().map(|watch| {
                    render_watch_item(state, watch, on_remove_watch)
                })}
            }
        </div>
    }
}

fn render_watch_item(
    state: &DebuggerState,
    watch: &Watch,
    on_remove_watch: &Callback<String>,
) -> Html {
    let stage_name = state
        .trace
        .as_ref()
        .and_then(|t| t.stage_names.get(watch.stage_index))
        .cloned()
        .unwrap_or_default();

    let next_stage_name = state
        .pipeline_lines
        .iter()
        .find(|l| l.stage_index == watch.stage_index + 1)
        .map(|l| l.text.split_whitespace().next().unwrap_or("").to_string())
        .unwrap_or_else(|| "END".to_string());

    let description = format!("after {stage_name} \u{2192} {next_stage_name}");

    let on_delete = {
        let cb = on_remove_watch.clone();
        let label = watch.label.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            cb.emit(label.clone());
        })
    };

    html! {
        <div class="watch-item">
            <div class="watch-item-header">
                <span class="watch-item-label">{&watch.label}</span>
                <span class="watch-item-desc">{description}</span>
                <button class="watch-delete" onclick={on_delete} title="Remove watch">
                    {"\u{1F5D1}"}
                </button>
            </div>
            <div class="watch-records">
                { render_watch_records(state, watch.stage_index) }
            </div>
        </div>
    }
}

fn render_watch_records(state: &DebuggerState, stage_index: usize) -> Html {
    if state.current_step == 0 {
        return html! {
            <span class="watch-not-reached">{"step to see data"}</span>
        };
    }

    let trace = match &state.trace {
        Some(t) => t,
        None => {
            return html! {
                <span class="watch-empty">{"no data"}</span>
            };
        }
    };

    let rc = trace.record_traces.len();
    let step = state.current_step - 1;
    let pp_index = stage_index + 1;

    let records = if step < rc {
        let rt = &trace.record_traces[step];
        rt.pipe_points.get(pp_index)
    } else {
        let flush_idx = step - rc;
        trace.flush_traces.get(flush_idx).and_then(|ft| {
            if stage_index >= ft.stage_index {
                let offset = stage_index - ft.stage_index + 1;
                ft.pipe_points.get(offset)
            } else {
                None
            }
        })
    };

    match records {
        Some(recs) if recs.is_empty() => {
            html! {
                <span class="watch-empty">{"(filtered out)"}</span>
            }
        }
        Some(recs) => {
            let count = recs.len();
            html! {
                <>
                    { for recs.iter().take(20).map(|r| {
                        html! {
                            <div class="watch-record">{r.as_str().trim_end()}</div>
                        }
                    })}
                    if count > 20 {
                        <div class="watch-record-more">
                            {format!("... ({count} total)")}
                        </div>
                    }
                </>
            }
        }
        None => {
            html! {
                <span class="watch-not-reached">{"not applicable"}</span>
            }
        }
    }
}
