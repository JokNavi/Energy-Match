use crate::{traits::patterns::{ContainsPattern, TargetPattern}};
use super::rows::Row;

pub const SIDE_AMOUNT: i32 = 4;
pub const LEVEL_SIZE: i32 = 50;
pub const TARGET_PATTERN_LENGTH: i32 = 3;
pub const DISPLAY_LENGTH: usize = 5;


pub struct Game {
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

    fn get_display(&self, index: i32) -> Result<(String, String), String>{
        if index > LEVEL_SIZE || index < 0 {return Err("Index out of bounds".to_string())}
        let mut horizontal_line = "...=".to_string();
        let mut middle_line = "   |".to_string();
        for i in 0..5{
            if let Some(value) = self.row.get_slice((index - 2) + i) {
                horizontal_line.push_str(&"=".repeat(7));
                middle_line.push_str(&format!(" [{value:^4}]"));
            } else {continue;};           
        }
        horizontal_line.push_str("==...");
        middle_line.push_str(" |   ");
        Ok((horizontal_line, middle_line))
    }

    pub fn display_row(&self, index: i32){
        if let Ok(lines) = self.get_display(index){
            println!("{}", lines.0);
            println!("{}", lines.1);
            println!("{}", lines.0);
        };
    }
}
