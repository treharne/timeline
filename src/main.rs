
use animations::{toggle_visible, push_subsequent_jobs};
use gloo_console::log;
use serde::{Deserialize, Serialize};
use yew::{prelude::*};


use gloo_storage::{Storage, LocalStorage};
use web_sys::{DragEvent};

mod line_components;
use crate::line_components::RunComponent;

mod colors;
use colors::get_color;

mod locations;
use locations::Location;

mod dnd;
use dnd::move_job;

mod animations;

type Minutes = usize;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Run {
    pub jobs: Vec<Job>,
    pub color: String,
    pub start_time: Minutes,
    pub end_time: Minutes,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Job {
    pub uid: String,
    pub color: String,
    pub location: Location,
    pub pushed: bool,
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AppState {
    pub runs: Vec<Run>,
    pub animate: bool,
}

type RunIdx = usize;
type ItemIdx = usize;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Position {
    pub run_idx: RunIdx,
    pub item_idx: ItemIdx,
}

impl Position {
    pub fn new(run_idx: RunIdx, item_idx: ItemIdx) -> Self {
        Self {run_idx, item_idx}
    }

    pub fn left_job_seq(&self) -> Option<usize> {
        if self.item_idx <= 0 {
            // There is no Job to the left of the first leg
            return None
        }
        Some((self.item_idx - 1) / 2)
    }

    pub fn is_leg(&self) -> bool {
        self.item_idx % 2 == 0
    }
}

pub enum Msg {
    DragStart(Position),
    Drop(Position),
    DragOver(Position),
    DragEnter(Position),
    DragLeave(Position),
    ToggleAnimations,
    Reset,
}

fn new_jobs(color: &str) -> Vec<Job> {
    let n = 10;
    (0..n)
        .map(|n| Job{
            uid: format!("{}", n),
            color: color.to_string(),
            location: Location::new_random(),
            pushed: false,
        }).collect()
}

fn new_runs() -> Vec<Run> {
    let n = 10;
    (0..n).map(|i| {
        let jobs = new_jobs(&get_color(i, n));
        Run { jobs: jobs, color: get_color(i, n), start_time: 0, end_time: 3 * 60 }}
    ).collect()
}

pub struct App {
    state: AppState,
    drag_from_pos: Option<Position>,
    dragging_over_pos: Option<Position>,
}


impl Component for App {
    type Message = Msg;
    type Properties = ();


    fn create(_ctx: &Context<Self>) -> Self {
        let runs = LocalStorage::get("timeline_state").unwrap_or_else(|_| new_runs());
        let state = AppState { runs, animate: false };

        App {
            state,
            drag_from_pos: None,
            dragging_over_pos: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DragStart(pos) => {
                self.drag_from_pos = Some(pos);
                self.dragging_over_pos = Some(pos);
                toggle_visible(&pos, false);
            }
            Msg::Drop(to_pos) => {
                let Some(from_pos) = self.drag_from_pos else { return false };
                if from_pos == to_pos { return false };

                move_job(from_pos, to_pos, &mut self.state);
                push_subsequent_jobs(&to_pos, false, &mut self.state.runs);

                match self.drag_from_pos {
                    Some(pos) => toggle_visible(&pos, true),
                    None => (),
                }
                
                LocalStorage::set("timeline_state", &self.state).unwrap();
                
                self.dragging_over_pos = None;
                self.drag_from_pos = None;
                return true;
            }

            Msg::DragEnter(pos) => {
                // - ondrag needed for cursour "grab" (prevent default or wthaver)
                match self.dragging_over_pos {
                    Some(dragging_over_pos) => {
                        if dragging_over_pos.run_idx == pos.run_idx {
                            // over the same run
                            push_subsequent_jobs(&pos, true, &mut self.state.runs);
                        } else {
                            // over a different run
                            let prev_run_pos = Position::new(dragging_over_pos.run_idx, 0);
                            push_subsequent_jobs(&prev_run_pos, false, &mut self.state.runs);
                            push_subsequent_jobs(&pos, true, &mut self.state.runs);
                        }},
                        None => {
                            // new run
                            push_subsequent_jobs(&pos, true, &mut self.state.runs);
                    }
                }

                self.dragging_over_pos = Some(pos);
                return true;
            },

            Msg::DragOver(_pos) => {
                return false;
            }

            Msg::DragLeave(_pos) => {
                return false;
            }
            Msg::Reset => {
                self.state.runs = new_runs();
                LocalStorage::delete("timeline_state");
                return true;
            }
            Msg::ToggleAnimations => {
                self.state.animate = !self.state.animate;
                return true;
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset = ctx.link().callback(|_| Msg::Reset);
        let drag_start = ctx.link().callback(move |(_, pos): (DragEvent, Position)| Msg::DragStart(pos));
        let drag_over = ctx.link().callback(move |(event, pos): (DragEvent, Position)| {
            event.prevent_default();
            Msg::DragOver(pos)
        });
        let drag_enter = ctx.link().callback(move |(event, pos): (DragEvent, Position)| {
            event.prevent_default();
            Msg::DragEnter(pos)
        });
        let drop = ctx.link().callback(move |(event, pos): (DragEvent, Position)| {
            event.prevent_default();
            Msg::Drop(pos)
        });
        let drag_leave = ctx.link().callback(move |(_, pos): (DragEvent, Position)| Msg::DragLeave(pos));

        let toggle_animations = ctx.link().callback(|_| Msg::ToggleAnimations);

        html! {
            <>
                { for self.state.runs.iter().enumerate().map(move|(run_idx, run)| html! {
                    <RunComponent 
                        run_idx={run_idx}
                        jobs={run.jobs.clone()}
                        color={run.color.clone()}
                        start_time={run.start_time}
                        end_time={run.end_time}
                        animate={self.state.animate}
                        drag_start={&drag_start}
                        drag_over={&drag_over}
                        drag_enter={&drag_enter}
                        drag_leave={&drag_leave}
                        drop={&drop}
                    />
                })}
                <br /><br />
                <label class="switch">
                    {"Animations:"} <input type="checkbox" checked={self.state.animate} onclick={toggle_animations} />
                    <span class="slider round"></span>
                </label>

                <br /><br />
                <button onclick={reset}>{"Reset"}</button>

            </>
        }
    }
}


pub fn main() {
    yew::Renderer::<App>::new().render();
}
