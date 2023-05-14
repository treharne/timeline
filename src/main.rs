
use std::str::FromStr;

use animations::{toggle_visible, push_subsequent_jobs};
use serde::{Deserialize, Serialize};
use strum::VariantNames;
use wasm_bindgen::JsCast;
use yew::{prelude::*};


use gloo_storage::{Storage, LocalStorage};
use web_sys::{HtmlInputElement};

mod line_components;
use crate::{line_components::RunComponent, dnd::CallbackMgr, animation_strategy::Strategy};

mod colors;
use colors::get_color;

mod locations;
use locations::Location;

mod dnd;
use dnd::move_job;

mod animations;
mod animation_strategy;

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
    pub pull: bool,
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AppState {
    pub runs: Vec<Run>,
    pub animation_strategy: Strategy,
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
        if self.item_idx == 0 {
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
    // ToggleAnimations,
    SetAnimationStrategy(Strategy),
    Reset,
}

fn new_jobs(color: &str) -> Vec<Job> {
    let n = 4;
    (0..n)
        .map(|n| Job{
            uid: format!("{}", n),
            color: color.to_string(),
            location: Location::new_random(),
            pushed: false,
            pull: false,
        }).collect()
}

fn new_runs() -> Vec<Run> {
    let n = 3;
    (0..n).map(|i| {
        let jobs = new_jobs(&get_color(i, n));
        Run { jobs, color: get_color(i, n), start_time: 0, end_time: 3 * 60 }}
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
        let state = AppState { runs, animation_strategy: Strategy::None };

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

                if let Some(pos) = self.drag_from_pos {
                    toggle_visible(&pos, true)
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
            Msg::SetAnimationStrategy(strategy) => {
                self.state.animation_strategy = strategy;
                return true;
            }
            // Msg::ToggleAnimations => {
            //     self.state.animate = !self.state.animate;
            //     return true;
            // }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset = ctx.link().callback(|_| Msg::Reset);
        let callback_mgr = CallbackMgr::new(ctx.link().clone());

        let on_input_change = ctx.link().callback(move |event: Event| {
            let Some(target) = event.target() else { return Msg::SetAnimationStrategy(Strategy::None) };
            let value = target.unchecked_into::<HtmlInputElement>().value();
            let Ok(selected) = value.parse::<String>() else { return Msg::SetAnimationStrategy(Strategy::None) };
            let strategy = Strategy::from_str(&selected).unwrap_or(Strategy::None);

            Msg::SetAnimationStrategy(strategy)
        });

        html! {
            <>
                { for self.state.runs.iter().enumerate().map(move|(run_idx, run)| html! {
                    <RunComponent 
                        run_idx={run_idx}
                        jobs={run.jobs.clone()}
                        color={run.color.clone()}
                        start_time={run.start_time}
                        end_time={run.end_time}
                        animation_strategy={self.state.animation_strategy.clone()}
                        callback_mgr={ callback_mgr.clone() }
                    />
                })}
                <br /><br />
                <div class="control">
                    <label class="radio">
                        { 
                            for Strategy::VARIANTS.iter().map(|strategy| html! {
                                <label>
                                    <input type="radio" name="anim" onchange={on_input_change.clone()} value={strategy.to_string()} checked={self.state.animation_strategy == Strategy::from_str(strategy).unwrap_or(Strategy::None)} />
                                    {strategy}
                                </label>
                        })}
                    </label>
                </div>
                    
                <br /><br />
                <button onclick={reset}>{"Reset"}</button>

            </>
        }
    }
}


pub fn main() {
    yew::Renderer::<App>::new().render();
}
