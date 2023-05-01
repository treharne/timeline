use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{Position, line_components::{make_run_id, make_item_id}};


// pub fn pull_subsequent_jobs(pos: &Position, pull: bool) {
//     let run_id = make_run_id(pos.run_idx);
//     if let Some(element) = document().get_element_by_id(&run_id) {
//         if let Some(run_html_element) = element.dyn_ref::<HtmlElement>() {
//             let item_idx = pos.item_idx + 1;
//             let n_items = run_html_element.children().length() as usize;
//             for idx in item_idx..n_items {
//                 let pos = Position::new(pos.run_idx, idx);
//                 // if pos.is_leg() { continue };
//                 pull_item(&pos, pull);
//             }
//         }
//     }
// }

pub fn push_subsequent_jobs(pos: &Position, push: bool) {
    let run_id = make_run_id(pos.run_idx);
    if let Some(element) = document().get_element_by_id(&run_id) {
        if let Some(run_html_element) = element.dyn_ref::<HtmlElement>() {
            let item_idx = pos.item_idx + 1;
            let n_items = run_html_element.children().length() as usize;
            for idx in item_idx..n_items {
                let pos = Position::new(pos.run_idx, idx);
                if pos.is_leg() { continue };
                push_item(&pos, push);
            }
        }
    }
}



fn push_item(pos: &Position, push: bool) {
    let element = get_item_at_pos(pos);
    if let Some(html_element) = element {
        set_class(&html_element, "push", push);
    }
}

pub fn toggle_visible(pos: &Position, visible: bool) {
    let element = get_item_at_pos(pos);
    if let Some(html_element) = element {
        set_class(&html_element, "hide", !visible);
    }
}

fn set_class(html_element: &HtmlElement, class_name: &str, set: bool) {
    let old_class = html_element.class_name();
    let new_class = if set {
        format!("{old_class} {class_name}")
    } else {
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