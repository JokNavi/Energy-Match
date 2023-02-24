use std::collections::HashMap;

use rand::Rng;

use super::row_slices::RowSlice;

pub struct Row {
    local_rotations: Vec<i32>,
    index: i32,
    slice_collection: HashMap<i32, RowSlice>,
}

impl Row {
    pub fn new(index: i32) -> Self {
        Self {
            local_rotations: Vec::new(),
            index,
            slice_collection: HashMap::from([(
                index,
                RowSlice::new(
                    rand::thread_rng().gen_range(0..=crate::SIDE_AMOUNT - 1),
                    index,
                ),
            )]),
        }
    }

    pub fn get_row_slice(&mut self, index: i32) -> &mut RowSlice {
        self.index = index;
        if let std::collections::hash_map::Entry::Vacant(e) = self.slice_collection.entry(index) {
            e.insert(RowSlice::new(
                rand::thread_rng().gen_range(0..=crate::SIDE_AMOUNT - 1),
                index,
            ));
            self.slice_collection
                .get_mut(&index)
                .expect("Just added the RowSlice to the list. ")
        } else {
            self.slice_collection
                .get_mut(&index)
                .expect("Just checked the RowSlice object exists in the list. ")
        }
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.local_rotations == other.local_rotations && self.index == other.index
    }
}

#[cfg(test)]
mod row_tests {

    #[test]
    fn new() {
        

    }
}

