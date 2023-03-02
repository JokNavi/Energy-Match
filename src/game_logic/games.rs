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
}

#[derive(Debug, PartialEq)]
enum UserChoiceError {
    InvalidSelection,
    InvalidAmount,
    InvalidConversion,
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

    fn parse_amount(choice: String) -> Result<i32, UserChoiceError>{
        match choice.split_whitespace().nth(1) {
            Some(value) => match value.parse::<i32>() {
                Ok(value) => Ok(value.abs()),
                Err(_) => return Err(UserChoiceError::InvalidAmount),
            },
            None => Err(UserChoiceError::InvalidAmount),
        }
    }

    fn parse_selection(choice: String) -> Result<UserChoice, UserChoiceError> {
        match choice.split_whitespace().next() {
            Some(value) => match value {
                "up" => Ok(UserChoice::Up),
                "down" => Ok(UserChoice::Down),
                "left" => Ok(UserChoice::Left),
                "right" => Ok(UserChoice::Right),
                _ => Err(UserChoiceError::InvalidSelection),
            },
            None => Err(UserChoiceError::InvalidSelection),
        }
    }

    fn check_choice(choice: String) ->  Result<(UserChoice, i32), UserChoiceError>  {
        if !choice.contains(" ") {
            return Err(UserChoiceError::InvalidConversion);
        }
        let amount = Self::parse_amount(choice.clone())?;        
        let selection = Self::parse_selection(choice)?;
        Ok((selection, amount))
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
            match Game::check_choice(choice){
                Ok((UserChoice::Down, amount)) => {println!("Swiping down..."); self.swipe_down(amount)},
                Ok((UserChoice::Up, amount)) => {println!("Swiping up..."); self.swipe_up(amount)},
                Ok((UserChoice::Left, amount)) => {println!("Swiping left..."); self.swipe_left(amount)},
                Ok((UserChoice::Right, amount)) => {println!("Swiping right..."); self.swipe_right(amount)},
                Err(UserChoiceError::InvalidAmount) => {println!("Couldn't proccess your inputted amount."); continue;},
                Err(UserChoiceError::InvalidConversion) => {println!("Couldn't convert your inputted amount to a valid format."); continue;},
                Err(UserChoiceError::InvalidSelection) => {println!("Couldn't proccess your inputted move."); continue;},
            };
            match row.display_row(3) {
                Ok(_) => (),
                Err(err) => match err {
                    RowIndexError::AboveMax => panic!("Index is out of bounds (too high)"),
                    RowIndexError::UnderZero => panic!("Index below 0"),
                    RowIndexError::NonI32Fitting => panic!("Can't be converted to i32"),
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
    use crate::{traits::indexes::CorrectIndex, game_logic::games::UserChoiceError};

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
        assert_eq!(Game::check_choice("up -1".to_string()),Ok((UserChoice::Up, 1)));
        assert_eq!(Game::check_choice("down 5".to_string()),Ok((UserChoice::Down, 5)));
        assert_eq!(Game::check_choice("left 1".to_string()),Ok((UserChoice::Left, 1)));
        assert_eq!(Game::check_choice("right 10".to_string()),Ok((UserChoice::Right, 10)));
        assert_eq!(Game::check_choice("up ".to_string()), Err(UserChoiceError::InvalidAmount));
        assert_eq!(Game::check_choice("asdf 1".to_string()), Err(UserChoiceError::InvalidSelection));
        assert_eq!(Game::check_choice("".to_string()), Err(UserChoiceError::InvalidConversion));
    }
}
