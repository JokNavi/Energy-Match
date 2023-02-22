use super::sides::Side;
use crate::data::details::{colors::color_selector, indexes::CorrectIndex};
use rand::Rng;
use std::collections::HashMap;

pub struct RowSlice {
    pub index: i32,
    pub rotations: i32,
    pub side_collection: HashMap<i32, Side>,
}

impl CorrectIndex for RowSlice{ }

impl RowSlice {
    pub fn new(index: i32) -> Self {
        let rotations = rand::thread_rng().gen_range(1..=crate::SIDE_AMOUNT);
        let mut side_collection: HashMap<i32, Side> = HashMap::new();
        side_collection.insert(rotations, Side::new(index, color_selector(rotations)));
        RowSlice {
            index,
            rotations,
            side_collection,
        }
    }

    pub fn get_side(&mut self) -> &mut Side {
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.side_collection.entry(self.rotations)
        {
            e.insert(Side::new(self.index, color_selector(self.rotations)));
            self.side_collection.get_mut(&self.rotations).unwrap()
        } else {
            self.side_collection.get_mut(&self.rotations).unwrap()
        }
    }

    
    pub fn swipe_up(&mut self, amount: i32) {
        self.rotations = Self::correct_side_index(self.rotations - amount);
    }

    pub fn swipe_down(&mut self, amount: i32) {
        self.rotations = Self::correct_side_index(self.rotations - amount);
    }

}



