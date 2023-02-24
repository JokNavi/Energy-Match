use std::collections::HashMap;

use rand::Rng;

use super::row_slices::RowSlice;

pub struct Row {
    local_rotations: Vec<i32>,
    slice_collection: HashMap<i32, RowSlice>,
}

//rand::thread_rng().gen_range(1..=crate::SIDE_AMOUNT);


impl Row {
    pub fn new() -> Self {
        let mut row_slice = RowSlice::new(rand::thread_rng().gen_range(1..=crate::SIDE_AMOUNT), 0);
        Self {
            local_rotations: row_slice.rotations,
            slice_collection: HashMap::from([(row_slice.index, row_slice)]),
        }
    }
}

