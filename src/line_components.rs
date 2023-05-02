use yew::prelude::*;
use gloo_console::log;
use crate::{Position, RunIdx, Job, locations::driving_time, Minutes};

#[derive(Properties, PartialEq)]
pub struct JobProps {
    pub pos: Position,
    pub label: String,
    pub color: String,
    pub duration: f32,
    pub pushed: bool,
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

pub fn make_run_id(run_idx: RunIdx) -> String {
    format!("run{}", run_idx)
}

#[function_component(JobComponent)]
pub fn job(props: &JobProps) -> Html {
    let JobProps { pos, label, color, duration, pushed, drag_start, drag_over, drag_leave, drop } = props;
    let style = to_style(vec![&border(color), &width(*duration, false)]);
    let class = if *pushed { "job push" } else { "job" };
    // let class = "job";
    html! {
        <div
            // `id` changes when the position changes,
            // you cannot use it as a permanent reference for this job.
            id={ make_item_id(pos) }
            uid={ label.clone() }
            class={ class }
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
    // pub pushed: bool,
    pub stretched: bool,
    pub drag_over: Callback<DragEvent>,
    pub drag_leave: Callback<DragEvent>,
    pub drop: Callback<DragEvent>,
}

#[function_component(LegComponent)]
pub fn leg(props: &LegProps) -> Html {
    let LegProps { pos, color, duration, stretched, drag_over, drag_leave, drop } = props;
    let style = to_style(vec![&bg(color), &width(*duration, false)]);
    let class = if *stretched { "leg stretch" } else { "leg" };
    // let class = "leg";
    html! {
        <div
            // `id` changes when the position changes,
            // you cannot use it as a permanent reference for this leg.
            id={ make_item_id(pos) }
            class={ class }
            ondragover={ drag_over }
            ondrop={ drop }
            ondragleave={ drag_leave }
            style={ style }
        />
    }
}

fn border(color: &str) -> String {
    let size = 5;
    format!("outline: solid {size}px {color}; outline-offset: -{size}px")
    // format!("border: {size}px solid {color}")
    // format!("border: {size}px solid {color}; box-sizing: border-box;")
    // format!("box-shadow:inset 0px 0px 0px {size}px {color}")
}

fn bg(color: &str) -> String {
    format!("background-color: {}", color)
    // format!("background-color: transparent")
}

fn width(duration: f32, stretched: bool) -> String {
    let item_width = (duration * 25.0).round() as u32;
    let width = if stretched { item_width + 50 } else { item_width };
    format!("width: {}px", width)
}

fn to_style(styles: Vec<&str>) -> String {
    styles.join("; ")
}

#[derive(Properties, PartialEq)]
pub struct RunProps {
    pub run_idx: RunIdx,
    pub jobs: Vec<Job>,
    pub color: String,
    pub start_time: Minutes,
    pub end_time: Minutes,
    pub drag_start: Callback<(DragEvent, Position)>,
    pub drag_over: Callback<(DragEvent, Position)>,
    pub drag_leave: Callback<(DragEvent, Position)>,
    pub drop: Callback<(DragEvent, Position)>,

}


fn render_job(pos: Position, job: &Job, run_props: &RunProps) -> Html {
    let drag_start = run_props.drag_start.reform(move |drag_event| (drag_event, pos));
    let drag_over = run_props.drag_over.reform(move |drag_event| (drag_event, pos));
    let drag_leave = run_props.drag_leave.reform(move |drag_event| (drag_event, pos));
    let drop = run_props.drop.reform(move |drag_event| (drag_event, pos));

    html! {
        <JobComponent
            pos={ pos.clone() }
            label={ job.uid.clone() }
            color={ job.color.clone() }
            duration={ 1.0 }
            pushed={ job.pushed }
            drag_start={ &drag_start }
            drag_over={ &drag_over }
            drag_leave={ &drag_leave }
            drop={ &drop }
        />
    }
}


fn render_leg(pos: Position, duration: f32, stretched: bool, run_props: &RunProps) -> Html {
    let drag_over = run_props.drag_over.reform(move |drag_event| (drag_event, pos));
    let drag_leave = run_props.drag_leave.reform(move |drag_event| (drag_event, pos));
    let drop = run_props.drop.reform(move |drag_event| (drag_event, pos));

    html! {
        <LegComponent
            pos={ pos.clone() }
            color={ run_props.color.clone() }
            duration={ duration }
            stretched={ stretched }
            drag_over={ &drag_over }
            drag_leave={ &drag_leave }
            drop={ &drop }
        />
    }
}

fn construct_run_elements(run_props: &RunProps) -> Vec<yew::virtual_dom::VNode> {
    let mut items = vec![];
    let run_idx = run_props.run_idx;
    
    let mut item_idx = 0;
    let pos = Position { run_idx, item_idx};
    let stretched = match run_props.jobs.first() {
        Some(job) => job.pushed,
        None => false,
    };
    let first_leg = render_leg(pos, 5.0, stretched, run_props);
    items.push(first_leg);


    let mut prev_job: Option<&Job> = None;
    for job in run_props.jobs.iter() {
        if prev_job.is_none() {
            item_idx += 1;
            let pos = Position{ run_idx, item_idx};
            items.push(render_job(pos, &job, run_props));
            prev_job = Some(job);
            continue
        }

        let prev = prev_job.unwrap();

        let leg_duration = driving_time(prev, job);
        
        item_idx += 1;
        let pos = Position { run_idx, item_idx };
        let stretched = job.pushed && !prev.pushed;
        items.push(render_leg(pos, leg_duration, stretched, run_props));
        
        item_idx += 1;
        let pos = Position { run_idx, item_idx };
        items.push(render_job(pos, &job, run_props));
        prev_job = Some(job);
    }

    item_idx += 1;
    let pos = Position { run_idx, item_idx };
    let last_leg = render_leg(pos, 5.0, false, run_props);
    items.push(last_leg);
    items
}    


#[function_component(RunComponent)]
pub fn run(props: &RunProps) -> Html {
    let day_length = props.end_time - props.start_time;
    let style = to_style(vec![&bg(&props.color), &width(day_length as f32, false)]);
    html! {
        <div class="run" id={ make_run_id(props.run_idx) } style={ style }>
            { for construct_run_elements(props) }
        </div>
    }
}