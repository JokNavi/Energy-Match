use super::sides::Side;
use crate::data::details::{colors::color_selector, indexes::CorrectIndex};
use rand::Rng;
use std::collections::HashMap;

pub struct RowSlice {
    pub selected_index: i32,
    pub rotations: i32,
    pub side_collection: HashMap<i32, Side>,
}

impl CorrectIndex for RowSlice {}

impl RowSlice {
    pub fn new(selected_index: i32) -> Self {
        let rotations = rand::thread_rng().gen_range(1..=crate::SIDE_AMOUNT);
        let mut side_collection: HashMap<i32, Side> = HashMap::new();
        side_collection.insert(
            rotations,
            Side::new(
                selected_index,
                color_selector(rotations).expect("value range is accounted for. "),
            ),
        );
        RowSlice {
            selected_index,
            rotations,
            side_collection,
        }
    }

    pub fn get_side(&mut self, rotations: i32) -> &mut Side {
        if let std::collections::hash_map::Entry::Vacant(e) = self.side_collection.entry(rotations)
        {
            e.insert(Side::new(
                self.selected_index,
                color_selector(rotations).expect("value range is accounted for. "),
            ));
            self.side_collection
                .get_mut(&rotations)
                .expect("Just added the Side to the list. ")
        } else {
            self.side_collection
                .get_mut(&rotations)
                .expect("Just checked the Side object exists in the list. ")
        }
    }

    pub fn get_current_side(&mut self) -> &mut Side {
        self.get_side(self.rotations)
    }
}
