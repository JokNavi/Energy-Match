use super::indexes::CorrectRanges;
use crate::data::game_logic::games::SIDE_AMOUNT;
use rand::Rng;

pub struct TargetPattern {
    pub pattern: Vec<i32>,
}
impl CorrectRanges for TargetPattern {}

impl TargetPattern {
    pub fn new(length: i32) -> Self {
        Self {
            pattern: Self::generate_pattern(length),
        }
    }

    fn generate_pattern(length: i32) -> Vec<i32> {
        (0..length)
            .map(|_| rand::thread_rng().gen_range(0..=SIDE_AMOUNT - 1))
            .collect()
    }

    pub fn set_pattern(&mut self, new_pattern: Vec<i32>) {
        self.pattern = new_pattern;
    }
}
