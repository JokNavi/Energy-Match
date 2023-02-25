use crate::data::visual_objects::{rows::Row, row_slices::RowSlice};

pub const SIDE_AMOUNT: i32 = 4;

struct Game {
    row: Row,
    moves_done: i32,
    target_pattern: Vec<i32>,
}

