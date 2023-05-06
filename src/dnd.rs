use crate::{Position, AppState, ItemIdx, App, Msg};
use gloo_console::log;
use web_sys::DragEvent;
use yew::{Callback, html::Scope};

#[derive(Clone, Debug)]
pub struct CallbackMgr {
    link: Scope<App>,
    _pos: Option<Position>,
}

impl PartialEq for CallbackMgr {
    fn eq(&self, other: &Self) -> bool {
        self._pos == other._pos
    }
}

impl CallbackMgr {
    pub fn new(link: Scope<App>) -> Self {
        Self {
            link: link,
            _pos: None,
        }
    }
    pub fn with_pos(&self, pos: Position) -> Self {
        Self {
            _pos: Some(pos),
            ..self.clone()
        }
    }
    fn pos(&self) -> Position {
        self._pos.expect("Cannot create callback without setting pos").clone()
    }
    pub fn drag_start(&self) -> Callback<DragEvent> {
        let pos = self.pos();
        self.link.callback(move |_: DragEvent| Msg::DragStart(pos))
    }
    pub fn drag_over(&self) -> Callback<DragEvent> {
        let pos = self.pos();
        self.link.callback(move |event: DragEvent| {
            event.prevent_default();
            Msg::DragOver(pos)
        })
    }
    pub fn drag_enter(&self) -> Callback<DragEvent> {
        let pos = self.pos();
        self.link.callback(move |event: DragEvent| {
            event.prevent_default();
            Msg::DragEnter(pos)
        })
    }
    pub fn drag_leave(&self) -> Callback<DragEvent> {
        let pos = self.pos();
        self.link.callback(move |_: DragEvent| Msg::DragLeave(pos))
    }
    pub fn drop(&self) -> Callback<DragEvent> {
        let pos = self.pos();
        self.link.callback(move |event: DragEvent| {
            event.prevent_default();
            Msg::Drop(pos)
        })
    }
}

pub fn move_job(from_pos: Position, to_pos: Position, state: &mut AppState) {
    log!(format!("Moving job from {:?} to {:?}", from_pos, to_pos));
    
    let runs = &mut state.runs;
    let from_run = runs.get_mut(from_pos.run_idx).unwrap();
    
    let from_job_seq = from_pos.left_job_seq().unwrap();
    let job = from_run.jobs.remove(from_job_seq);
    
    let to_run = if from_pos.run_idx == to_pos.run_idx {
        log!(format!("Moving to same run"));
        from_run
    } else {
        runs.get_mut(to_pos.run_idx).unwrap()
    };
    
    let insert_idx = calc_insertion_idx(&from_pos, &to_pos);
    log!(format!("Moving job from_job_seq {:?} insert_seq {:?}", from_job_seq, insert_idx));
    to_run.jobs.insert(insert_idx, job);
}


fn calc_insertion_idx(from_pos: &Position, to_pos: &Position) -> ItemIdx {
    let later_in_same_run = (from_pos.run_idx == to_pos.run_idx) 
                                && (from_pos.item_idx < to_pos.item_idx);

    match to_pos.left_job_seq() {
        Some(seq) => if later_in_same_run { seq } else { seq + 1 },
        None => 0,
    }
}
