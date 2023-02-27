use crate::{details::patterns::{ContainsPattern, TargetPattern}};
use super::rows::Row;

pub const SIDE_AMOUNT: i32 = 4;
pub const LEVEL_SIZE: i32 = 50;
pub const TARGET_PATTERN_LENGTH: i32 = 3;
pub const DISPLAY_LENGTH: usize = 5;


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

    pub fn display(&self, index: i32) -> Result<(), String>{
        if index > LEVEL_SIZE || index < 0 {return Err("Index out of bounds".to_string())}
        let mut horizontal_line = "...=".to_string();
        let mut middle_row = "  |".to_string();
        for i in 0..5{
            if let Some(value) = self.row.get_slice((index as usize - 2) + i) {
                horizontal_line.push_str(&"=".repeat(DISPLAY_LENGTH * 7));
                middle_row.push_str(&format!(" [{value}]"));
            } else {break};           
        }
        horizontal_line.push_str("==...");
        middle_row.push_str(" |   ");
        Ok(())
    }

    /* 
    fn middle_row(self, index: usize) {
        let start_adjuster = (1.index)

    }
    */ 
}
