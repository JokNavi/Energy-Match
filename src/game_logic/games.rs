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

    fn get_edge_line(&self, index: i32) -> String {
        let mut edge_line = "...=".to_string();
        for i in 0..DISPLAY_LENGTH {
            let usize_index = match Self::validate_index((index - 2) + i as i32) {
                Ok(value) => value,
                Err(err) => continue,
            };
            edge_line.push_str(&"=".repeat(7));
        }
        edge_line.push_str("==...");
        edge_line
    }

    fn get_display_line(&self, index: i32) -> String 
    {
        let mut display_line = "   |".to_string();
        for i in 0..DISPLAY_LENGTH {
            let usize_index = match Self::validate_index((index - 2) + i as i32) {
                Ok(value) => value,
                Err(err) => continue,
            };
            let value = self.row.get_slice(usize_index).unwrap();
            display_line.push_str(&format!(" [{value:^4}]"));
        }
        display_line.push_str(" |   ");
        display_line
    }

    pub fn display_row<T>(&self, index: T) -> Result<(), String>
    where
        T: TryInto<i32> + TryInto<usize> + Copy,
    {
        let index: i32 = Self::validate_index(index)?.try_into().unwrap();
        let display_line = self.get_display_line(index);
        let edge_line = self.get_edge_line(index);
        println!("{}", edge_line);
        println!("{}", display_line);
        println!("{}", edge_line);
        Ok(())
    }
}

#[cfg(test)]
mod test_game {
    use crate::game_logic::games::LEVEL_SIZE;
    use super::Game;

    #[test]
    fn get_edge_line() {
        let game = Game::new();
        assert_eq!(game.get_edge_line(-1), "...=================...");
        assert_eq!(game.get_edge_line(LEVEL_SIZE), "...=================...");

        assert_eq!(game.get_edge_line(3), "...======================================...");
        assert_eq!(game.get_edge_line(1), "...===============================...");
        assert_eq!(game.get_edge_line(0), "...========================...");

        assert_eq!(game.get_edge_line(LEVEL_SIZE - 3), "...======================================...");
        assert_eq!(game.get_edge_line(LEVEL_SIZE - 2), "...===============================..."); 
        assert_eq!(game.get_edge_line(LEVEL_SIZE - 1), "...========================...");
    }

    #[test]
    fn display_row() {
        let game = Game::new();
        assert_eq!(game.display_row(-1), Err("Index below 0".to_string()));
        assert_eq!(game.display_row(0), Ok(()));
        assert_eq!(game.display_row(LEVEL_SIZE), Err("Index is out of bounds (too high)".to_string()));
    }
}
