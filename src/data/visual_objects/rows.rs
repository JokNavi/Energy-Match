use crate::data::details::indexes::GenerateSlices;
use crate::data::game_logic::games::LEVEL_SIZE;

pub struct Row {
    slices: Vec<i32>,
    index: i32,
}

impl GenerateSlices for Row {}

impl Row {
    pub fn new() -> Self {
        Self {
            slices: Self::generate_slices(LEVEL_SIZE),
            index: 0,
        }
    }

    pub fn get_slice(&self, index: usize) -> Option<i32> {
        self.slices.get(index).copied()
    }
}
