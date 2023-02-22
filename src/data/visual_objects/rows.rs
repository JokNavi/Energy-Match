use super::row_slices::RowSlice;
use crate::data::details::indexes::CorrectIndex;
use std::collections::{HashMap, HashSet};

pub struct RowDisplay {
    row: String,
    center_index: i32,
    width: i32,
}

pub struct Row {
    pub selected_index: i32,
    pub rotation_stack: HashMap<i32, i32>,
    pub row_slice_collection: HashMap<i32, RowSlice>,
}

impl CorrectIndex for Row {}

impl Row {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            rotation_stack: HashMap::new(),
            row_slice_collection: HashMap::new(),
        }
    }

    pub fn get_row_slice(&mut self, index: i32) -> &mut RowSlice {
        if let std::collections::hash_map::Entry::Vacant(e) = self.row_slice_collection.entry(index)
        {
            e.insert(RowSlice::new(index));
            self.row_slice_collection
                .get_mut(&index)
                .expect("Just added the RowSlice to the list. ")
        } else {
            self.row_slice_collection
                .get_mut(&index)
                .expect("Just checked the RowSlice object exists in the list. ")
        }
    }

    pub fn get_current_row_slice(&mut self) -> &mut RowSlice {
        self.get_row_slice(self.selected_index)
    }

    pub fn rotation_stack_insert(&mut self, amount: i32) {
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.rotation_stack.entry(self.selected_index)
        {
            e.insert(amount);
        } else {
            *self
                .rotation_stack
                .get_mut(&self.selected_index)
                .expect("I just checked it exists. ") += amount;
        }
    }

    pub fn swipe_up(&mut self, amount: i32) {
        self.rotation_stack_insert(amount);
        self.get_current_row_slice().rotations =
            Self::correct_side_index(self.get_current_row_slice().rotations + amount);
    }

    pub fn swipe_down(&mut self, amount: i32) {
        self.rotation_stack_insert(-amount);
        self.get_current_row_slice().rotations =
            Self::correct_side_index(self.get_current_row_slice().rotations - amount);
    }

    pub fn swipe_left(&mut self, amount: i32){
        let range = Self::get_range(self.selected_index, self.selected_index + amount);
        self.rotation_stack.retain(|i, _| range.contains(i));
        self.selected_index = self.selected_index + amount;
        self.get_current_row_slice().rotations = self.rotation_stack.values().sum();
    }

    pub fn swipe_right(&mut self, amount: i32){
        let range = Self::get_range(self.selected_index, self.selected_index - amount);
        self.rotation_stack.retain(|i, _| range.contains(i));
        self.selected_index = self.selected_index - amount;
        self.get_current_row_slice().rotations = self.rotation_stack.values().sum();
    }
}
