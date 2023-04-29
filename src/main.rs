
use colors::get_color;
use serde::{Deserialize, Serialize};
use yew::{prelude::*};

use gloo_storage::{Storage, LocalStorage};
use web_sys::{DragEvent};

mod line_components;
use crate::line_components::RunComponent;

mod colors;
use crate::colors::PALETTES;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Run {
    items: Vec<RunItem>,
    color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct AppState {
    runs: Vec<Run>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RunItem {
    Job(String),
    Leg,
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
}

pub enum Msg {
    DragStart(Position),
    Drop(Position),
    DragOver(Position),
    DragLeave(Position),
    Reset,
}

fn new_items() -> Vec<RunItem> {
    let mut items = vec![RunItem::Leg];
    let others: Vec<_> = (1..=10).map(|n| [RunItem::Job(format!("Box {}", n)), RunItem::Leg]).flatten().collect();
    items.extend(others);
    items
}

fn new_runs() -> Vec<Run> {
    let n = 10;
    (0..n).map(|i| Run { items: new_items(), color: get_color(i, n) }).collect()
}
fn move_job(from_pos: Position, to_pos: Position, state: &mut AppState) {
    // order is important for:
    // - adding to to_item_idx must be done before subtracting -> usize can't be negative
    // - removing from run.items must be done before inserting -> if it's the same run, the indexes move
    // - removing Job then Leg must be done before inserting Leg then Job

    gloo_console::log!(format!("Moving job from {:?} to {:?}", from_pos, to_pos));
    let from_item_idx = from_pos.item_idx;
    let mut to_item_idx = to_pos.item_idx;
        
    let runs = &mut state.runs;
    let from_run = runs.get_mut(from_pos.run_idx).unwrap();
    
    let job = from_run.items.remove(from_item_idx);
    let leg = from_run.items.remove(from_item_idx);

    let to_run = if from_pos.run_idx == to_pos.run_idx {
        let move_earlier = to_item_idx < from_item_idx;
        to_item_idx += if move_earlier { 2 } else { 0 };
        gloo_console::log!(format!("Moving to same run"));
        from_run
    } else {
        to_item_idx += 2;
        runs.get_mut(to_pos.run_idx).unwrap()
    };

    let dropped_onto = to_run.items.get(to_pos.item_idx).unwrap();
    to_item_idx -= match dropped_onto {
        RunItem::Leg => 1,
        RunItem::Job(_) => 0,
    };

    gloo_console::log!(format!("Moving job from_item_idx {:?} to_item_idx {:?}", from_item_idx, to_item_idx));
    to_run.items.insert(to_item_idx, leg);
    to_run.items.insert(to_item_idx, job);
}

pub struct App {
    state: AppState,
    drag_item_index: Option<Position>,
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
            drag_item_index: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DragStart(pos) => {
                self.drag_item_index = Some(pos);
            }
            Msg::Drop(to_pos) => {
                let Some(from_pos) = self.drag_item_index else { return false };
                if from_pos == to_pos { return false };

                move_job(from_pos, to_pos, &mut self.state);
                LocalStorage::set("timeline_state", &self.state).unwrap();
                return true;
            }

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
                        run_items={run.items.clone()}
                        color={run.color.clone()}
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
