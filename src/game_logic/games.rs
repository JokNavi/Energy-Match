use super::rows::Row;
use crate::traits::{
    indexes::CorrectIndex,
    patterns::TargetPattern,
};

pub const SIDE_AMOUNT: i32 = 4;

pub const TARGET_PATTERN_LENGTH: i32 = 3;
pub const DISPLAY_LENGTH: usize = 5;

pub struct Game {
    row: Row,
    moves_done: i32,
    target_pattern: TargetPattern,
}

impl CorrectIndex for Game {}

impl Game {
    pub fn new() -> Self {
        Self {
            row: Row::new(50),
            moves_done: 0,
            target_pattern: TargetPattern::new(TARGET_PATTERN_LENGTH),
        }
    }

    pub fn swipe_left(&mut self, amount: i32) {
        if let Ok(index) = Self::validate_index(self.row.index - amount) {
            self.row.index = index as i32;
        }
    }

    pub fn swipe_right(&mut self, amount: i32) {
        if let Ok(index) = Self::validate_index(self.row.index + amount) {
            self.row.index = index as i32;
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_game {
    use super::Game;
}
