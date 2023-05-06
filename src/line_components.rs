use yew::prelude::*;
use crate::{Position, RunIdx, Job, locations::driving_time, Minutes, dnd::CallbackMgr};


#[derive(Properties, PartialEq)]
pub struct JobProps {
    pub pos: Position,
    pub label: String,
    pub color: String,
    pub duration: f32,
    pub pushed: bool,
    pub animate: bool,
    pub callback_mgr: CallbackMgr,
}

pub fn make_item_id(pos: &Position) -> String {
    // creates an id for an item based on its position only
    // so get_element_by_id can be used while dragging.
    format!("item-run{}-seq{}", pos.run_idx, pos.item_idx)
}

pub fn make_run_id(run_idx: RunIdx) -> String {
    format!("run{}", run_idx)
}

fn job_class(push: bool, animate: bool) -> String {
    if push && animate {
        "job push-animate"
    } else if push {
        "job push"
    } else {
        "job"
    }.to_string()
}

#[function_component(JobComponent)]
pub fn job(props: &JobProps) -> Html {
    let style = to_style(vec![&border(&props.color), &width(props.duration)]);
    let class = job_class(props.pushed, props.animate);
    // let callback_mgr = props.callback_mgr.with_pos(props.pos);
    html! {
        <div
            // `id` changes when the position changes,
            // you cannot use it as a permanent reference for this job.
            id={ make_item_id(&props.pos) }
            uid={ props.label.clone() }
            class={ class }
            draggable={ "true" }
            ondragstart={ &props.callback_mgr.drag_start() }
            ondragover={ &props.callback_mgr.drag_over() }
            ondragenter={ &props.callback_mgr.drag_enter() }
            ondragleave={ &props.callback_mgr.drag_leave() }
            ondrop={ &props.callback_mgr.drop() }
            
            style={ style }
        >
            { &props.label }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LegProps {
    pub pos: Position,
    pub color: String,
    pub duration: f32,
    pub stretched: bool,
    pub pushed: bool,
    pub animate: bool,
    pub callback_mgr: CallbackMgr,
}

fn leg_class(stretch: bool, push: bool, animate:bool) -> String {
    if stretch && animate {
        "leg stretch-animate"
    } else if push && animate {
        "leg push-animate"
    } else if stretch {
        "leg stretch"
    } else if push {
        "leg push"
    } else {
        "leg"
    }.to_string()
}

#[function_component(LegComponent)]
pub fn leg(props: &LegProps) -> Html {
    let style = to_style(vec![
        &width(props.duration), 
        &leg_scale_ratio(props.duration),
    ]);

    let class = leg_class(props.stretched, props.pushed, props.animate);
                      
    html! {
        <div
            // `id` changes when the position changes,
            // you cannot use it as a permanent reference for this leg.
            id={ make_item_id(&props.pos) }
            class={ class }
            ondragover={ &props.callback_mgr.drag_over() }
            ondragenter={ &props.callback_mgr.drag_enter() }
            ondrop={ &props.callback_mgr.drop() }
            ondragleave={ &props.callback_mgr.drag_leave() }

            style={ style }
        />
    }
}

fn border(color: &str) -> String {
    let size = 5;
    format!("outline: solid {size}px {color}; outline-offset: -{size}px")
}

fn bg(color: &str) -> String {
    format!("background-color: {}", color)
    // format!("background-color: transparent")
}

fn _width(duration: f32) -> u32 {
    (duration * 25.0).round() as u32
}

fn width(duration: f32) -> String {
    let width = _width(duration);
    format!("width: {width}px")
}

fn leg_scale_ratio(duration: f32) -> String {
    let width = _width(duration);
    let ratio = ((width + 50) as f32) / (width as f32);
    format!("--scale-ratio: {ratio}")
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
    pub animate: bool,
    pub callback_mgr: CallbackMgr,
}


fn render_job(pos: Position, job: &Job, animate: bool, run_props: &RunProps) -> Html {
    let callback_mgr = run_props.callback_mgr.with_pos(pos.clone());

    html! {
        <JobComponent
            pos={ pos.clone() }
            label={ job.uid.clone() }
            color={ job.color.clone() }
            duration={ 1.0 }
            pushed={ job.pushed }
            animate={ animate }
            callback_mgr={ callback_mgr }
        />
    }
}


fn render_leg(pos: Position, duration: f32, stretched: bool, pushed: bool, animate: bool, run_props: &RunProps) -> Html {
    let callback_mgr = run_props.callback_mgr.with_pos(pos.clone());

    html! {
        <LegComponent
            pos={ pos.clone() }
            color={ run_props.color.clone() }
            duration={ duration }
            stretched={ stretched }
            pushed={ pushed }
            animate={ animate }
            callback_mgr={ callback_mgr }
        />
    }
}

fn construct_run_elements(run_props: &RunProps) -> Vec<yew::virtual_dom::VNode> {
    let animate = run_props.animate;
    let mut items = vec![];
    let run_idx = run_props.run_idx;
    
    let mut item_idx = 0;
    let pos = Position { run_idx, item_idx};
    let stretched = match run_props.jobs.first() {
        Some(job) => job.pushed,
        None => false,
    };
    let first_leg = render_leg(pos, 5.0, stretched, false, animate, run_props);
    items.push(first_leg);


    let mut prev_job: Option<&Job> = None;
    for job in run_props.jobs.iter() {
        if prev_job.is_none() {
            item_idx += 1;
            let pos = Position{ run_idx, item_idx};
            items.push(render_job(pos, &job, animate, run_props));
            prev_job = Some(job);
            continue
        }

        let prev = prev_job.unwrap();

        let leg_duration = driving_time(prev, job);
        
        item_idx += 1;
        let pos = Position { run_idx, item_idx };
        let stretched = job.pushed && !prev.pushed;
        items.push(render_leg(pos, leg_duration, stretched, prev.pushed, animate, run_props));
        
        item_idx += 1;
        let pos = Position { run_idx, item_idx };
        items.push(render_job(pos, &job, animate, run_props));
        prev_job = Some(job);
    }

    item_idx += 1;
    let pos = Position { run_idx, item_idx };
    let pushed = match prev_job {
        Some(job) => job.pushed,
        None => false,
    };
    let last_leg = render_leg(pos, 5.0, false, pushed, animate, run_props);
    items.push(last_leg);
    items
}    


#[function_component(RunComponent)]
pub fn run(props: &RunProps) -> Html {
    let day_length = props.end_time - props.start_time;
    let style = to_style(vec![&bg(&props.color), &width(day_length as f32)]);
    html! {
        <div class="run" id={ make_run_id(props.run_idx) } style={ style }>
            { for construct_run_elements(props) }
        </div>
    }
}