use yew::prelude::*;

use crate::{Position, RunIdx};

#[derive(Properties, PartialEq)]
pub struct JobProps {
    pub pos: Position,
    pub label: String,
    pub color: String,
    pub duration: u32,
    pub drag_start: Callback<DragEvent>,
    pub drag_over: Callback<DragEvent>,
    pub drag_leave: Callback<DragEvent>,
    pub drop: Callback<DragEvent>,
}

#[function_component(JobComponent)]
pub fn job(props: &JobProps) -> Html {
    let JobProps { pos, label, color, duration, drag_start, drag_over, drag_leave, drop } = props;
    let style = to_style(vec![&border(color), &width(*duration)]);
    html! {
        <div
            uid={ label.clone() }
            class="item"
            draggable={ "true" }
            ondragstart={ drag_start }
            ondragover={ drag_over }
            ondrop={ drop }
            ondragleave={ drag_leave }
            style={ style }
        >
            { label }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LegProps {
    pub pos: Position,
    pub color: String,
    pub duration: u32,
    pub drag_over: Callback<DragEvent>,
    pub drag_leave: Callback<DragEvent>,
    pub drop: Callback<DragEvent>,
}

#[function_component(LegComponent)]
pub fn leg(props: &LegProps) -> Html {
    let LegProps { pos, color, duration, drag_over, drag_leave, drop } = props;
    let style = to_style(vec![&bg(color), &width(*duration)]);
    html! {
        <div
            // id={format!("item-{}", index)}
            class="placeholder"
            ondragover={ drag_over }
            ondrop={ drop }
            ondragleave={ drag_leave }
            style={ style }
        >
        { " " }
        </div>
    }
}


fn border(color: &str) -> String {
    format!("border: 2px solid {color}")
}

fn bg(color: &str) -> String {
    format!("background-color: {}", color)
}

fn width(duration: u32) -> String {
    format!("width: {}px", duration * 10)
}

fn to_style(styles: Vec<&str>) -> String {
    styles.join("; ")
}

#[derive(Properties, PartialEq)]
pub struct RunProps {
    pub run_idx: RunIdx,
    pub run_items: Vec<crate::RunItem>,
    pub drag_start: Callback<(DragEvent, Position)>,
    pub drag_over: Callback<(DragEvent, Position)>,
    pub drag_leave: Callback<(DragEvent, Position)>,
    pub drop: Callback<(DragEvent, Position)>,

}


fn render_run_item(pos: Position, item: &crate::RunItem, run_props: &RunProps) -> Html {
    let drag_start = run_props.drag_start.reform(move |drag_event| (drag_event, pos));
    let drag_over = run_props.drag_over.reform(move |drag_event| (drag_event, pos));
    let drag_leave = run_props.drag_leave.reform(move |drag_event| (drag_event, pos));
    let drop = run_props.drop.reform(move |drag_event| (drag_event, pos));

    match item {
        crate::RunItem::Job(job_str) => {
            html! {
                <JobComponent
                    pos={ pos }
                    label={ job_str.clone() }
                    color={ "#34495e".to_string() }
                    duration={4}
                    drag_start={ &drag_start }
                    drag_over={ &drag_over }
                    drag_leave={ &drag_leave}
                    drop={ &drop }
                />
            }
        }
        crate::RunItem::Leg => {
            html! {
                <LegComponent
                    pos={ pos }
                    color={ "#ff804d".to_string() }
                    duration={3}
                    drag_over={ &drag_over }
                    drag_leave={ &drag_leave}
                    drop={ &drop }
                />
            }
        }
    }
}

#[function_component(RunComponent)]
pub fn run(props: &RunProps) -> Html {
    let RunProps { run_idx, run_items, drag_start, drag_over, drag_leave, drop } = props;
    html! {
        <div class="run">
            { for run_items.iter().enumerate().map(|(item_idx, item)| {
                let pos = Position { run_idx: *run_idx, item_idx };
                render_run_item(pos, item, props)
            }) }
        </div>
    }
}