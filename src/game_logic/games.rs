use super::rows::Row;
use crate::traits::{indexes::CorrectIndex, patterns::TargetPattern};

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
        if let Ok(index) = Self::validate_index(self.row.index - amount.abs()) {
            self.row.index = index as i32;
        }
    }

    pub fn swipe_right(&mut self, amount: i32) {
        if let Ok(index) = Self::validate_index(self.row.index + amount.abs()) {
            self.row.index = index as i32;
        }
    }

    pub fn swipe_up(&mut self, amount: i32 ) {
        if let Some(value) = self.row.get_slice(self.row.index){
            self.row.set_slice(value + amount);
        }
    } 

    pub fn swipe_down(&mut self, amount: i32 ) {
        if let Some(value) = self.row.get_slice(self.row.index){
            self.row.set_slice(value - amount);
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

    use super::Game;

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
        assert_eq!(game.row.get_slice(0).unwrap(), Game::adjust_rotation(prev_value + 1));
    }

    #[test]
    pub fn swipe_down() {
        let mut game = Game::default();
        let mut prev_value = game.row.get_slice(0).unwrap();
        game.swipe_down(1);
        assert_eq!(game.row.get_slice(0).unwrap(), Game::adjust_rotation(prev_value - 1));
    }
}
