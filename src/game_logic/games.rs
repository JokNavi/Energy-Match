use crate::{Details::patterns::{ContainsPattern, TargetPattern}};
use super::rows::Row;


pub const SIDE_AMOUNT: i32 = 4;
pub const LEVEL_SIZE: i32 = 50;
pub const TARGET_PATTERN_LENGTH: i32 = 3;

struct Game {
    row: Row,
    moves_done: i32,
    target_pattern: TargetPattern,
}

impl ContainsPattern for Game {
    fn contains_pattern(&self, pattern: Vec<i32>) -> bool {
        for window in self.row.slices.windows(pattern.len()) {
            if window == pattern.as_slice() {
                return true;
            }
        }
        false
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            row: Row::new(LEVEL_SIZE),
            moves_done: 0,
            target_pattern: TargetPattern::new(TARGET_PATTERN_LENGTH),
        }
    }

    
}
