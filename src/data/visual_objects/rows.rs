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
        let index =rand::thread_rng().gen_range(1..=crate::SIDE_AMOUNT);
        Self {
            local_rotations: Vec::new(),
            slice_collection: HashMap::from([(index, RowSlice::new(index, 0))]),
        }
    }
}

