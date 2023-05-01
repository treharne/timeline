use yew::prelude::*;

use crate::{Position, RunIdx};

#[derive(Properties, PartialEq)]
pub struct JobProps {
    pub pos: Position,
    pub label: String,
    pub color: String,
    pub duration: f32,
    pub drag_start: Callback<DragEvent>,
    pub drag_over: Callback<DragEvent>,
    pub drag_leave: Callback<DragEvent>,
    pub drop: Callback<DragEvent>,
}

pub fn make_item_id(pos: &Position) -> String {
    // creates an id for an item based on its position only
    // so get_element_by_id can be used while dragging.
    format!("item-run{}-seq{}", pos.run_idx, pos.item_idx)
}

#[function_component(JobComponent)]
pub fn job(props: &JobProps) -> Html {
    let JobProps { pos, label, color, duration, drag_start, drag_over, drag_leave, drop } = props;
    let style = to_style(vec![&border(color), &width(*duration)]);
    html! {
        <div
            // `id` changes when the position changes,
            // you cannot use it as a permanent reference for this job.
            id={ make_item_id(pos) }
            uid={ label.clone() }
            class="job"
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
    pub duration: f32,
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
            // `id` changes when the position changes,
            // you cannot use it as a permanent reference for this leg.
            id={ make_item_id(pos) }
            class="leg"
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

fn width(duration: f32) -> String {
    let width = (duration * 30.0).round() as u32;
    format!("width: {}px", width)
}

fn to_style(styles: Vec<&str>) -> String {
    styles.join("; ")
}

#[derive(Properties, PartialEq)]
pub struct RunProps {
    pub run_idx: RunIdx,
    pub run_items: Vec<crate::RunItem>,
    pub color: String,
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
        crate::RunItem::Job(job) => {
            html! {
                <JobComponent
                    pos={ pos }
                    label={ job.uid.clone() }
                    color={ "#34495e".to_string() }
                    duration={ 1.0 }
                    drag_start={ &drag_start }
                    drag_over={ &drag_over }
                    drag_leave={ &drag_leave }
                    drop={ &drop }
                />
            }
        }
        crate::RunItem::Leg(leg) => {
            html! {
                <LegComponent
                    pos={ pos }
                    color={ run_props.color.clone() }
                    duration={ leg.duration }
                    drag_over={ &drag_over }
                    drag_leave={ &drag_leave }
                    drop={ &drop }
                />
            }
        }
    }
}

#[function_component(RunComponent)]
pub fn run(props: &RunProps) -> Html {
    html! {
        <div class="run">
            { for props.run_items.iter().enumerate().map(|(item_idx, item)| {
                let pos = Position { run_idx: props.run_idx, item_idx };
                render_run_item(pos, item, props)
            }) }
        </div>
    }
}