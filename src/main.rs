use data::details::patterns::TargetPattern;

use crate::data::{visual_objects::row_slices::RowSlice, details::indexes::CorrectIndex};

mod data;

const SIDE_AMOUNT: i32 = 4;
const ALLOW_0_INDEX: bool = false;

fn main() {

    println!("TargetPattern: {}", TargetPattern::new(std::ops::Range { start: 1, end: 5 }));
    let mut row_slice = RowSlice::new(0);
    println!("{}", RowSlice::correct_side_index(5));
}
