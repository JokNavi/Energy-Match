use super::rows::Row;
use crate::traits::{
    indexes::CorrectIndex,
    patterns::{ContainsPattern, TargetPattern},
};

pub const SIDE_AMOUNT: i32 = 4;
pub const LEVEL_SIZE: i32 = 50;
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
            row: Row::new(LEVEL_SIZE),
            moves_done: 0,
            target_pattern: TargetPattern::new(TARGET_PATTERN_LENGTH),
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
