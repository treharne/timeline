
use animations::{push_subsequent_jobs, toggle_visible, push_subsequent_jobs2};
use gloo_utils::document;
use gloo_console::log;
use line_components::{make_item_id, make_run_id};
use serde::{Deserialize, Serialize};
use yew::{prelude::*};


use gloo_storage::{Storage, LocalStorage};
use web_sys::{DragEvent, HtmlElement, HtmlCollection};
use wasm_bindgen::JsCast;

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
    pub runs: Vec<Run>
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
    DragLeave(Position),
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
    let n = 20;
    (0..n).map(|i| {
        let jobs = new_jobs(&get_color(i, n));
        Run { jobs: jobs, color: get_color(i, n), start_time: 0, end_time: 8 * 60 }}
    ).collect()
}

pub struct App {
    state: AppState,
    drag_from_pos: Option<Position>,
    dragging_over_pos: Option<Position>,
}

// fn set_pos_draggable(pos: &Position, draggable: bool) {
//     let item_id = make_item_id(pos);
//     if let Some(element) = document().get_element_by_id(&item_id) {
//         if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
//             html_element.set_draggable(draggable);
//         }
//     }
// }



impl Component for App {
    type Message = Msg;
    type Properties = ();


    fn create(_ctx: &Context<Self>) -> Self {
        let runs = LocalStorage::get("timeline_state").unwrap_or_else(|_| new_runs());
        let state = AppState { runs };

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
                // push_subsequent_jobs(&to_pos, false);
                push_subsequent_jobs2(&to_pos, false, &mut self.state.runs);

                match self.drag_from_pos {
                    Some(pos) => toggle_visible(&pos, true),
                    None => (),
                }
                
                LocalStorage::set("timeline_state", &self.state).unwrap();
                
                self.dragging_over_pos = None;
                self.drag_from_pos = None;
                return true;
            }

            Msg::DragOver(pos) => {
                // Should consider using ondragenter instead of dragover... probably way faster
                // - ondrag needed for cursour "grab" (prevent default or wthaver)
                // need to calculate when/how to give the "contract" class to a leg...
                // consider using .leg::hover { ...css... } https://stackoverflow.com/questions/16516793/how-to-reverse-an-animation-on-mouse-out-after-hover
                match self.dragging_over_pos {
                    Some(dragging_over_pos) => {
                        if pos == dragging_over_pos {
                            // still over the same item
                            // This early return MASSIVELY improves performance
                            return false
                        } else if dragging_over_pos.run_idx == pos.run_idx {
                            // over the same run
                            // push_subsequent_jobs(&pos, true);
                            push_subsequent_jobs2(&pos, true, &mut self.state.runs);
                        } else {
                            // over a different run
                            let prev_run_pos = Position::new(dragging_over_pos.run_idx, 0);
                            // push_subsequent_jobs(&prev_run_pos, false);
                            // push_subsequent_jobs(&pos, true);
                            push_subsequent_jobs2(&prev_run_pos, false, &mut self.state.runs);
                            push_subsequent_jobs2(&pos, true, &mut self.state.runs);
                        }},
                        None => {
                            // new run
                            // push_subsequent_jobs(&pos, true);
                            push_subsequent_jobs2(&pos, true, &mut self.state.runs);
                    }
                }

                self.dragging_over_pos = Some(pos);
                return true;
            }
            Msg::DragLeave(pos) => {
                log!(format!("Drag Leave {:?}", pos));
                // match self.dragging_over_run {
                //     Some(run_idx) => if run_idx == pos.run_idx {
                //         // over the same run
                //     },
                //     None => {
                //         // left run
                //         let pos = Position::new(pos.run_idx, 0);
                //         push_subsequent_jobs(&pos, false);
                //     }
                // }
                // self.dragging_over_run = Some(pos.run_idx);
                // push_subsequent_jobs(&pos, false);
                return true;
            }
            Msg::Reset => {
                self.state.runs = new_runs();
                LocalStorage::delete("timeline_state");
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
        let drop = ctx.link().callback(move |(event, pos): (DragEvent, Position)| {
            event.prevent_default();
            Msg::Drop(pos)
        });
        let drag_leave = ctx.link().callback(move |(_, pos): (DragEvent, Position)| Msg::DragLeave(pos));

        html! {
            <>
                { for self.state.runs.iter().enumerate().map(move|(run_idx, run)| html! {
                    <RunComponent 
                        run_idx={run_idx}
                        jobs={run.jobs.clone()}
                        color={run.color.clone()}
                        start_time={run.start_time}
                        end_time={run.end_time}
                        drag_start={&drag_start}
                        drag_over={&drag_over}
                        drag_leave={&drag_leave}
                        drop={&drop}
                    />
                })}
                <button onclick={reset}>{"Reset"}</button>
            </>
        }
    }
}


pub fn main() {
    yew::Renderer::<App>::new().render();
}
