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

    fn get_edge_line(&self, index: usize) -> String {
        let mut index: usize;
        let mut edge_line = "...=".to_string();
        for i in 0..DISPLAY_LENGTH {
            index = match Self::validate_index((index - 2) + i) {
                Ok(value) => value,
                Err(err) => continue,
            };
            edge_line.push_str(&"=".repeat(7));
        }
        edge_line.push_str("==...");
        edge_line
    }

    fn get_display_line(&self, index: usize) -> String {
        let mut index: usize;
        let mut display_line = "   |".to_string();
        for i in 0..DISPLAY_LENGTH {
            index = match Self::validate_index((index - 2) + i) {
                Ok(value) => value,
                Err(err) => continue,
            };
            let value = self.row.get_slice((index - 2) + i).unwrap();
            display_line.push_str(&format!(" [{value:^4}]"));
        }
        display_line.push_str(" |   ");
        display_line
    }

    pub fn display_row<T>(&self, index: T)
    where
        T: TryInto<i32> + TryInto<usize> + Copy,
    {
        let index = match Self::validate_index(index) {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
        let display_line = self.get_display_line(index);
        let edge_line = self.get_edge_line(index);
        println!("{}", edge_line);
        println!("{}", display_line);
        println!("{}", edge_line);
    }
}

#[cfg(test)]
mod TestGame {
    use super::Game;
    use super::LEVEL_SIZE;

    #[test]
    fn get_display() {
        let game = Game::new();
        assert_eq!(
            game.get_display(3).unwrap().0,
            "...======================================..."
        );
        assert_eq!(
            game.get_display(0).unwrap().0,
            "...========================..."
        );
        assert_eq!(
            game.get_display(LEVEL_SIZE - 1).unwrap().0,
            "...========================..."
        );

        assert_eq!(game.get_display(LEVEL_SIZE - 1).unwrap().1.len(), 30);
        assert_eq!(game.get_display(0).unwrap().1.len(), 30);
        assert_eq!(game.get_display(3).unwrap().1.len(), 44);
    }
}
