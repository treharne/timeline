use yew::prelude::*;
use crate::{Position, RunIdx, Job, locations::driving_time, Minutes, dnd::CallbackMgr, animation_strategy::{Strategy, get_classes}};

#[derive(Properties, PartialEq)]
pub struct JobProps {
    pub pos: Position,
    pub label: String,
    pub color: String,
    pub duration: f32,
    pub pushed: bool,
    pub pull: bool,
    pub animation_strategy: Strategy,
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

fn job_class(push: bool, strategy: Strategy) -> String {
    let (_, push_approach) = strategy.parts();
    let (push_base, pushed) = push_approach.push_classes();
    "job".to_owned() + &push_base + if push {&pushed} else {""}
}

#[function_component(JobComponent)]
pub fn job(props: &JobProps) -> Html {
    let style = to_style(vec![&border(&props.color), &width(props.duration)]);
    let class = job_class(props.pushed, props.animation_strategy.clone());
    
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
    pub animation_strategy: Strategy,
    pub callback_mgr: CallbackMgr,
}

fn leg_class(stretch: bool, push: bool, strategy: Strategy) -> String {
    let (base, stretched, pushed) = get_classes(strategy);
    "leg".to_owned() + &base + if stretch {&stretched} else if push {&pushed} else {""}
}

#[function_component(LegComponent)]
pub fn leg(props: &LegProps) -> Html {
    let style = to_style(vec![
        &width(props.duration), 
        &leg_scale_vars(props.duration),
    ]);

    let class = leg_class(props.stretched, props.pushed, props.animation_strategy.clone());
                      
    html! {
        <div
            // `id` changes when the position changes,
            // you cannot use it as a permanent reference for this leg.
            id={ make_item_id(&props.pos) }
            class={ class }
            style={ style }
            ondragover={ &props.callback_mgr.drag_over() }
            ondragenter={ &props.callback_mgr.drag_enter() }
            ondrop={ &props.callback_mgr.drop() }
            ondragleave={ &props.callback_mgr.drag_leave() }
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

fn leg_scale_vars(duration: f32) -> String {
    let width = _width(duration);
    let stretch_width = width + 50;
    let stretch_ratio = ((stretch_width) as f32) / (width as f32);
    format!("--scale-width: {stretch_width}px; --scale-ratio: {stretch_ratio}")
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
    pub animation_strategy: Strategy,
    pub callback_mgr: CallbackMgr,
}


fn render_job(pos: Position, job: &Job, run_props: &RunProps) -> Html {
    let callback_mgr = run_props.callback_mgr.with_pos(pos);

    html! {
        <JobComponent
            pos={ pos }
            label={ job.uid.clone() }
            color={ job.color.clone() }
            duration={ 1.0 }
            pushed={ job.pushed }
            pull={ job.pull }
            animation_strategy={ run_props.animation_strategy.clone() }
            callback_mgr={ callback_mgr }
        />
    }
}


fn render_leg(pos: Position, duration: f32, stretched: bool, pushed: bool, run_props: &RunProps) -> Html {
    let callback_mgr = run_props.callback_mgr.with_pos(pos);

    html! {
        <LegComponent
            pos={ pos }
            color={ run_props.color.clone() }
            duration={ duration }
            stretched={ stretched }
            pushed={ pushed }
            animation_strategy={ run_props.animation_strategy.clone() }
            callback_mgr={ callback_mgr }
        />
    }
}

fn construct_run_elements(run_props: &RunProps) -> Vec<yew::virtual_dom::VNode> {
    let mut items = vec![];
    let run_idx = run_props.run_idx;
    
    let mut item_idx = 0;
    let pos = Position { run_idx, item_idx };
    let stretched = run_props.jobs.first().map_or(false, |job| job.pushed);
    let first_leg = render_leg(pos, 5.0, stretched, false, run_props);
    items.push(first_leg);

    let mut prev_job = None;
    for job in run_props.jobs.iter() {
        let prev = match prev_job {
            Some(j) => j,
            None => {
                // It's the first job in the run
                item_idx += 1;
                let pos = Position{ run_idx, item_idx };
                items.push(render_job(pos, job, run_props));
                prev_job = Some(job);
                continue
            }
        };

        let leg_duration = driving_time(prev, job);
        
        item_idx += 1;
        let pos = Position { run_idx, item_idx };
        let stretched = job.pushed && !prev.pushed;
        items.push(render_leg(pos, leg_duration, stretched, prev.pushed, run_props));
        
        item_idx += 1;
        let pos = Position { run_idx, item_idx };
        items.push(render_job(pos, job, run_props));
        prev_job = Some(job);
    }

    item_idx += 1;
    let pos = Position { run_idx, item_idx };
    let pushed = prev_job.map_or(false, |job| job.pushed);
    let last_leg = render_leg(pos, 5.0, false, pushed, run_props);
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