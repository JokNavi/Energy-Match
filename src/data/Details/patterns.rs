use super::indexes::{CorrectRanges, GenerateSlices};

pub struct TargetPattern {
    pub pattern: Vec<i32>,
}
impl CorrectRanges for TargetPattern {}

impl GenerateSlices for TargetPattern{}

impl TargetPattern {
    pub fn new(length: i32) -> Self {
        Self {
            pattern: Self::generate_slices(length),
        }
    }

    pub fn set_pattern(&mut self, new_pattern: Vec<i32>) {
        self.pattern = new_pattern;
    }
}
