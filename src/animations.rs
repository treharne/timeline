use gloo_utils::document;
use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{Position, line_components::{make_item_id}, Run, Job};

pub fn push_subsequent_jobs(pos: &Position, push: bool, runs: &mut [Run]) -> Option<()> {
    let run = runs.get_mut(pos.run_idx)?;
    let jobs: &mut Vec<Job> = run.jobs.as_mut();

    let right_job_idx = match pos.left_job_seq() {
        Some(seq) => seq + 1,
        None => 0,
    };
    
    log!(format!("push_subsequent_jobs2: job_idx: {}", right_job_idx));
    for (idx, job) in jobs.iter_mut().enumerate() {
        if idx < right_job_idx { 
            job.pushed = false;
            job.pull = true;
        } else {
            job.pull = !push;
            job.pushed = push;
        }
    };
    Some(())
}


// // pub fn pull_subsequent_jobs(pos: &Position, pull: bool) {
// //     let run_id = make_run_id(pos.run_idx);
// //     if let Some(element) = document().get_element_by_id(&run_id) {
// //         if let Some(run_html_element) = element.dyn_ref::<HtmlElement>() {
// //             let item_idx = pos.item_idx + 1;
// //             let n_items = run_html_element.children().length() as usize;
// //             for idx in item_idx..n_items {
// //                 let pos = Position::new(pos.run_idx, idx);
// //                 // if pos.is_leg() { continue };
// //                 pull_item(&pos, pull);
// //             }
// //         }
// //     }
// // }

// // Idea:
// // instead of pushing jobs which come after the current one,
// // just elongate the next leg (using a css transition).
// // This should push all subsequent jobs, and give a bigger
// // drop zone.


// // pub fn push_subsequent_jobs(pos: &Position, push: bool) {
// //     let run_id = make_run_id(pos.run_idx);
// //     if let Some(element) = document().get_element_by_id(&run_id) {
// //         if let Some(run_html_element) = element.dyn_ref::<HtmlElement>() {
// //             let item_idx = pos.item_idx;
// //             let n_items = run_html_element.children().length() as usize;
// //             for idx in 0..n_items {
// //                 if idx != item_idx {
// //                     let pos = Position::new(pos.run_idx, idx);
// //                     push_item(&pos, push);
// //                 }
// //                 // let pos = Position::new(pos.run_idx, idx);


// //                 // if idx <= item_idx {
// //                 //     push_item(&pos, false);
// //                 //     continue
// //                 // } 
// //                 // push_item(&pos, push);
// //             }
// //         }
// //     }
// // }

// pub fn push_subsequent_jobs(pos: &Position, push: bool) {
//     let run_id = make_run_id(pos.run_idx);
//     if let Some(element) = document().get_element_by_id(&run_id) {
//         if let Some(run_html_element) = element.dyn_ref::<HtmlElement>() {
//             let item_idx = pos.item_idx;
//             let n_items = run_html_element.children().length() as usize;

//             let from = 0.max(item_idx.wrapping_sub(3));
//             let to = n_items.min(item_idx + 3);


//             // for idx in 0..n_items {
//             for idx in from..to {
//                 let pos = Position::new(pos.run_idx, idx);
//                 if pos.is_leg() { continue };
                
//                 let el = run_html_element.children().item(idx as u32).unwrap();
//                 let html_el = el.dyn_ref::<HtmlElement>();
//                 if let Some(html_el) = html_el {
//                     if idx <= item_idx {
//                         set_class(&html_el, "push", false);
//                         // push_item(&pos, false);
//                         continue
//                     } 
//                     set_class(&html_el, "push", push);
//                     // push_item(&pos, push);
//                 }
//             }

//             // for idx in 0..n_items {
//             //     let pos = Position::new(pos.run_idx, idx);
//             //     if pos.is_leg() { continue };
//             //     if idx <= item_idx {
//             //         push_item(&pos, false);
//             //         continue
//             //     } 
//             //     push_item(&pos, push);
//             // }
//         }
//     }
// }

// // fn widen_leg(pos: &Position, widen: bool) {
// //     let element = get_item_at_pos(pos);
// //     if let Some(html_element) = element {
// //         set_class(&html_element, "widen", widen);
// //         set_class(&html_element, "unwiden", !widen);
// //     }
// // }

// fn push_item(pos: &Position, push: bool) {
//     let element = get_item_at_pos(pos);
//     if let Some(html_element) = element {
//         set_class(&html_element, "push", push);
//         // set_class(&html_element, "unpush", !push);
//     }
// }

// pub fn toggle_visible(pos: &Position, visible: bool, runs: &mut Vec<Run>) {
//     let run = runs.get_mut(pos.run_idx)?;
//     let jobs: &mut Vec<Job> = run.jobs.as_mut();

//     let job_idx = match pos.left_job_seq() {
//         Some(seq) => seq,
//         None => return,
//     };

//     let job = match jobs.get_mut(job_idx) {
//         Some(job) => job,
//         None => return,
//     };

//     job.visible = visible;
// }

pub fn toggle_visible(pos: &Position, visible: bool) {
    let element = get_item_at_pos(pos);
    if let Some(html_element) = element {
        set_class(&html_element, "hide", !visible);
    }
}

fn set_class(html_element: &HtmlElement, class_name: &str, set: bool) {
    let class_name = &format!(" {}", class_name);
    let old_class = html_element.class_name();
    let new_class = if set {
        if old_class.contains(class_name) { return };
        format!("{old_class}{class_name}")
    } else {
        if !old_class.contains(class_name) { return };
        // let old_class = ;
        old_class.replace(class_name, "").trim().to_string()
    };

    html_element.set_class_name(&new_class)
}

fn get_item_at_pos(pos: &Position) -> Option<HtmlElement> {
    let item_id = make_item_id(pos);
    if let Some(element) = document().get_element_by_id(&item_id) {
        if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
            return Some(html_element.clone());
        }
    }
    None
}