use std::io::{self, Write};

use super::rows::Row;
use crate::traits::{self, indexes::CorrectIndex, indexes::RowIndexError, patterns::TargetPattern};

pub const SIDE_AMOUNT: i32 = 4;
pub const TARGET_PATTERN_LENGTH: i32 = 3;
pub const DISPLAY_LENGTH: usize = 5;

pub struct Game {
    row: Row,
    moves_done: i32,
    target_pattern: TargetPattern,
}

impl CorrectIndex for Game {}

#[derive(Debug, PartialEq)]
enum UserChoice {
    Up,
    Down,
    Left,
    Right,
    InvalidSelection,
    InvalidAmount,
    Invalid,
}

impl Game {
    pub fn new() -> Self {
        Self {
            row: Row::new(50),
            moves_done: 0,
            target_pattern: TargetPattern::new(TARGET_PATTERN_LENGTH),
        }
    }

    fn swipe_left(&mut self, amount: i32) {
        if let Ok(index) = Self::validate_index(self.row.index - amount.abs()) {
            self.row.index = index as i32;
        }
    }

    fn swipe_right(&mut self, amount: i32) {
        if let Ok(index) = Self::validate_index(self.row.index + amount.abs()) {
            self.row.index = index as i32;
        }
    }

    fn swipe_up(&mut self, amount: i32) {
        if let Some(value) = self.row.get_slice(self.row.index) {
            self.row.set_slice(value + amount);
        }
    }

    fn swipe_down(&mut self, amount: i32) {
        if let Some(value) = self.row.get_slice(self.row.index) {
            self.row.set_slice(value - amount);
        }
    }

    fn ask_user(prompt: &str) -> String {
        print!("{}: ", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        input.trim().to_string()
    }

    fn check_choice(choice: String) -> (UserChoice, Option<i32>) {
        if !choice.contains(" ") {
            return (UserChoice::Invalid, None);
        }
        let mut choice = choice.split_whitespace();

        let move_choice = match choice.next() {
            Some("up") => UserChoice::Up,
            Some("down") => UserChoice::Down,
            Some("left") => UserChoice::Left,
            Some("right") => UserChoice::Right,
            Some(_) => UserChoice::InvalidSelection,
            None => UserChoice::InvalidSelection,
        };

        let amount: Option<i32> = match choice.next() {
            Some(value) => match value.parse::<i32>() {
                Ok(value) => Some(value),
                Err(_) => return (UserChoice::InvalidAmount, None),
            },
            None => return (UserChoice::InvalidAmount, None),
        };

        (move_choice, amount)
        
    }

    pub fn start(&mut self) {
        let mut choice = String::new();
        let row = Row::new(50);
        println!("Starting game...");
        println!("Type \"up x\" to swipe up x times.");
        println!("Type \"down x\" to swipe down x times.");
        println!("Type \"left x\" to swipe left x times.");
        println!("Type \"right x\" to swipe right x times.");
        row.display_row(3).unwrap();
        loop {
            choice = Self::ask_user("Please choose an option. (or type \"quit\" to quit)");
            if choice.contains(" ") {}

            match row.display_row(3) {
                Ok(_) => (),
                Err(err) => match err {
                    RowIndexError::AboveMax => println!("Index is out of bounds (too high)"),
                    RowIndexError::UnderZero => println!("Index below 0"),
                    RowIndexError::NonI32Fitting => println!("Can't be converted to i32"),
                },
            };
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
    use crate::traits::indexes::CorrectIndex;

    use super::{Game, UserChoice};

    #[test]
    pub fn swipe_right() {
        let mut game = Game::default();
        assert_eq!(game.row.index, 0);

        game.swipe_right(5);
        assert_eq!(game.row.index, 5);

        game.swipe_right(44);
        assert_eq!(game.row.index, 49);

        game.swipe_right(1);
        assert_eq!(game.row.index, 49);
    }

    #[test]
    pub fn swipe_left() {
        let mut game = Game::default();

        game.swipe_right(10);
        assert_eq!(game.row.index, 10);

        game.swipe_left(5);
        assert_eq!(game.row.index, 5);

        game.swipe_left(5);
        assert_eq!(game.row.index, 0);

        game.swipe_left(1);
        assert_eq!(game.row.index, 0);
    }

    #[test]
    pub fn swipe_up() {
        let mut game = Game::default();
        let mut prev_value = game.row.get_slice(0).unwrap();
        game.swipe_up(1);
        assert_eq!(
            game.row.get_slice(0).unwrap(),
            Game::adjust_rotation(prev_value + 1)
        );
    }

    #[test]
    pub fn swipe_down() {
        let mut game = Game::default();
        let mut prev_value = game.row.get_slice(0).unwrap();
        game.swipe_down(1);
        assert_eq!(
            game.row.get_slice(0).unwrap(),
            Game::adjust_rotation(prev_value - 1)
        );
    }

    #[test]
    pub fn check_choice() {
        assert_eq!(Game::check_choice("up 1".to_string()), (UserChoice::Up, Some(1)));       
        assert_eq!(Game::check_choice("down 1".to_string()), (UserChoice::Down, Some(1)));
        assert_eq!(Game::check_choice("left 1".to_string()), (UserChoice::Left, Some(1)));
        assert_eq!(Game::check_choice("right 1".to_string()), (UserChoice::Right, Some(1)));
        assert_eq!(Game::check_choice("up1".to_string()), (UserChoice::Invalid, None));
        assert_eq!(Game::check_choice("asdfs 1".to_string()), (UserChoice::InvalidSelection, Some(1)));
        assert_eq!(Game::check_choice("up aa".to_string()), (UserChoice::InvalidAmount, None));
    }
}
