use data::visual_objects::rows::Row;

mod data;

const SIDE_AMOUNT: i32 = 4;

fn main() {
    let mut row = Row::new();
    let index_0 = row.get_row_slice(0).get_current_side().display_value.color_text();
    let index_1 = row.get_row_slice(1).get_current_side().display_value.color_text();

    println!("index 0: {}, index 1: {}", index_0, index_1);
    row.swipe_left(1);
    println!("index 0: {}", row.get_row_slice(0).get_current_side().display_value);
    row.swipe_up(1);
    println!("{}", row.get_current_row_slice().get_current_side().display_value);
    
}
