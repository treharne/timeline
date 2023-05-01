use crate::{Position, Run, RunItem, AppState};

pub fn move_job(from_pos: Position, to_pos: Position, state: &mut AppState) {
    // order is important for:
    // - adding to to_item_idx must be done before subtracting -> usize can't be negative
    // - removing from run.items must be done before inserting -> if it's the same run, the indexes move
    // - removing Job then Leg must be done before inserting Leg then Job

    gloo_console::log!(format!("Moving job from {:?} to {:?}", from_pos, to_pos));
    
    let runs = &mut state.runs;
    let modified_to_pos = calculate_modified_to_pos(&from_pos, &to_pos, &runs);
    let from_run = runs.get_mut(from_pos.run_idx).unwrap();
    
    let job = from_run.items.remove(from_pos.item_idx);
    let leg = from_run.items.remove(from_pos.item_idx);

    let to_run = if from_pos.run_idx == to_pos.run_idx {
        gloo_console::log!(format!("Moving to same run"));
        from_run
    } else {
        runs.get_mut(to_pos.run_idx).unwrap()
    };

    gloo_console::log!(format!("Moving job from_pos.item_idx {:?} modified_to_pos.item_idx {:?}", from_pos.item_idx, modified_to_pos.item_idx));
    to_run.items.insert(modified_to_pos.item_idx, leg);
    to_run.items.insert(modified_to_pos.item_idx, job);
}


fn calculate_modified_to_pos(from_pos: &Position, to_pos: &Position, runs: &Vec<Run>) -> Position {
    let mut offset: isize = 0;
    
    if from_pos.run_idx == to_pos.run_idx {
        let move_earlier = to_pos.item_idx < from_pos.item_idx;
        offset += if move_earlier { 2 } else { 0 };
    } else {
        offset += 2;
    };
    
    let to_run = runs.get(to_pos.run_idx).unwrap();
    let dropped_onto = to_run.items.get(to_pos.item_idx).unwrap();
    offset -= match dropped_onto {
        RunItem::Leg{..} => 1,
        _ => 0,
    };

    Position::new(
        to_pos.run_idx, 
        to_pos.item_idx.wrapping_add_signed(offset),
    )
}