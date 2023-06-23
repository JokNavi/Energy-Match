use rand::{thread_rng, Rng};

const MIN_LOWEST_GAME_VALUE: i8 = 0; // can change.
const MIN_HIGHEST_GAME_VALUE: i8 = MIN_LOWEST_GAME_VALUE + 1;
const MIN_SPECIAL_VALUE_INDEX: i8 = 0;
const MIN_WINDOW_HEIGHT: i8 = MIN_HIGHEST_GAME_VALUE;
const MIN_GAME_VALUE_AMOUNT: i8 = 1; // can change
const MIN_MOVE_AMOUNT: i8 = 1; // can change

const MAX_HIGHEST_GAME_VALUE: i8 = 99; // can change
const MAX_LOWEST_GAME_VALUE: i8 = i8::MAX - MAX_HIGHEST_GAME_VALUE;
const MAX_SPECIAL_VALUE_INDEX: i8 = MAX_GAME_VALUE_AMOUNT;
const MAX_WINDOW_HEIGHT: i8 = MAX_HIGHEST_GAME_VALUE;
const MAX_GAME_VALUE_AMOUNT: i8 = 20; // can change
const MAX_MOVE_AMOUNT: i8 = 127; // can change


//for testing.
const DEFAULT_LOWEST_GAME_VALUE: i8 = 1; // can change
const DEFAULT_HIGHEST_GAME_VALUE: i8 = 8; // can change
const DEFAULT_SPECIAL_VALUE_INDEX: i8 = 6;
const DEFAULT_WINDOW_HEIGHT: i8 = 3; // can change
const DEFAULT_GAME_VALUE_AMOUNT: i8 = 11; // can change
const DEFAULT_MOVE_AMOUNT: i8 = 1; // can change

pub type GameValues = Vec<i8>;
pub type GameWindowView = Vec<GameValues>;

#[derive(PartialEq, Debug)]
pub enum GameWindowError {
    NewGameWindowError(NewGameWindowError),
    SwipeError(SwipeError),
}

#[derive(PartialEq, Debug)]
pub enum NewGameWindowError {
    LowestGameValueTooLow,
    LowestGameValueTooHigh,
    HighestGameValueTooLow,
    HighestGameValueTooHigh,
    WindowHeightTooLow,
    WindowHeightTooHigh,
    SomeCurrentGameValuesTooLow,
    SomeCurrentGameValuesTooHigh,
    TooFewGameValues,
    TooManyGameValues,
    SpecialValueIndexTooLow,
    SpecialValueIndexTooHigh,
}

#[derive(PartialEq, Debug)]
pub enum SwipeError {
    IndexTooLow,
    IndexTooHigh,
    MoveAmountTooLow,
    MoveAmountTooHigh,
}

#[derive(Debug, Clone)]
pub struct GameWindow {
    highest_game_value: i8,
    lowest_game_value: i8,
    special_value_index: i8,
    window_height: i8,
    game_value_amount: i8,
    current_game_values: GameValues,
}

impl GameWindow {
    pub fn new(
        highest_game_value: i8,
        lowest_game_value: i8,
        special_value_index: i8,
        window_height: i8,
        current_game_values: GameValues,
    ) -> Result<Self, GameWindowError> {
        match highest_game_value {
            _ if highest_game_value < MIN_HIGHEST_GAME_VALUE => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::HighestGameValueTooLow,
                ));
            }
            _ if highest_game_value > MAX_HIGHEST_GAME_VALUE => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::HighestGameValueTooHigh,
                ));
            }
            _ => (),
        };

        match lowest_game_value {
            _ if lowest_game_value < MIN_LOWEST_GAME_VALUE => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::LowestGameValueTooLow,
                ));
            }
            _ if lowest_game_value > MAX_LOWEST_GAME_VALUE => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::LowestGameValueTooHigh,
                ));
            }
            _ => (),
        };

        match window_height {
            _ if window_height < MIN_WINDOW_HEIGHT => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::WindowHeightTooLow,
                ));
            }
            _ if window_height > MAX_WINDOW_HEIGHT => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::WindowHeightTooHigh,
                ));
            }
            _ => (),
        };

        match current_game_values {
            _ if !current_game_values
                .iter()
                .all(|value| value >= &lowest_game_value) =>
            {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::SomeCurrentGameValuesTooLow,
                ));
            }
            _ if !current_game_values
                .iter()
                .all(|value| value <= &highest_game_value) =>
            {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::SomeCurrentGameValuesTooHigh,
                ));
            }
            _ if ((current_game_values.len() as i32) < (MIN_GAME_VALUE_AMOUNT as i32)) => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::TooFewGameValues,
                ));
            }
            _ if ((current_game_values.len() as i32) > (MAX_GAME_VALUE_AMOUNT as i32)) => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::TooManyGameValues,
                ));
            }
            _ => (),
        };

        let game_value_amount = current_game_values.len() as i8;

        match special_value_index {
            _ if special_value_index < MIN_SPECIAL_VALUE_INDEX => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::SpecialValueIndexTooLow,
                ));
            }

            _ if special_value_index > MAX_SPECIAL_VALUE_INDEX => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::SpecialValueIndexTooHigh,
                ));
            }

            _ if special_value_index >= game_value_amount => {
                return Err(GameWindowError::NewGameWindowError(
                    NewGameWindowError::SpecialValueIndexTooHigh,
                ));
            }
            _ => (),
        };

        Ok(Self {
            highest_game_value,
            lowest_game_value,
            special_value_index,
            window_height,
            game_value_amount,
            current_game_values,
        })
    }

    pub fn correct_game_value(&self, value: i8) -> i8 {
        let value = value - self.lowest_game_value;
        let game_value_range_length = (self.highest_game_value - self.lowest_game_value) + 1;
        let index = value % game_value_range_length;
        if index < 0 {
            return self.highest_game_value + index + 1;
        }
        self.lowest_game_value + index
    }

    pub fn view(&self) -> GameWindowView {
        let half_window_size = self.window_height / 2;
        let mut view: GameWindowView = Vec::with_capacity(self.window_height as usize);
        (1..=half_window_size).for_each(|i| {
            view.push(
                self.current_game_values
                    .iter()
                    .map(|game_value| self.correct_game_value(game_value - i))
                    .collect::<GameValues>(),
            )
        });
        view.push(self.current_game_values.clone());
        (1..=half_window_size).for_each(|i| {
            view.push(
                self.current_game_values
                    .iter()
                    .map(|game_value| self.correct_game_value(game_value + i))
                    .collect::<GameValues>(),
            )
        });
        view
    }

    pub fn validate_swipe_parameters(&mut self, index: i32, amount: i32) -> Result<(i8, i8), GameWindowError>  {
        let index = match index {
            _ if index < MIN_GAME_VALUE_AMOUNT as i32 => return Err(GameWindowError::SwipeError(SwipeError::IndexTooLow)),
            _ if index < 0 as i32 => return Err(GameWindowError::SwipeError(SwipeError::IndexTooLow)),
            _ if index >= self.game_value_amount as i32 => return Err(GameWindowError::SwipeError(SwipeError::IndexTooHigh)),
            _ => index as i8,
       };

       let amount = match amount {
           _ if amount < MIN_MOVE_AMOUNT as i32 => return Err(GameWindowError::SwipeError(SwipeError::MoveAmountTooLow)),
           _ if amount > MAX_MOVE_AMOUNT as i32 => return Err(GameWindowError::SwipeError(SwipeError::MoveAmountTooHigh)),
           _ => amount as i8,
       };
       
       Ok((index, amount))
    }

    pub fn swipe_up(&mut self, index: i32, amount: i32) -> Result<(), GameWindowError> {
        let (index, amount) = self.validate_swipe_parameters(index, amount)?;
        match index {
           _ if index < self.special_value_index => self.current_game_values[0..=index as usize].iter_mut().for_each(|game_value|  *game_value += amount),
           _ if index == self.special_value_index => self.current_game_values[index as usize] += amount,
           _ if index > self.special_value_index => self.current_game_values[index as usize..self.game_value_amount as usize].iter_mut().for_each(|game_value| *game_value += amount),
           _ => unreachable!()
        }
        Ok(())
    }

    pub fn swipe_down(&mut self, index: i32, amount: i32) -> Result<(), GameWindowError> {
        let (index, amount) = self.validate_swipe_parameters(index, amount)?;
        match index {
           _ if index < self.special_value_index => self.current_game_values[0..=index as usize].iter_mut().for_each(|game_value|  *game_value -= amount),
           _ if index == self.special_value_index => self.current_game_values[index as usize] -= amount,
           _ if index > self.special_value_index => self.current_game_values[index as usize..self.game_value_amount as usize].iter_mut().for_each(|game_value| *game_value -= amount),
           _ => unreachable!()
        }
        Ok(())
    }
}

pub trait InitGameStruct {
    fn randomise_game_values(&mut self);
}

impl InitGameStruct for GameWindow {
    fn randomise_game_values(&mut self) {
        let mut rng = thread_rng();
        self.current_game_values = (0..=self.game_value_amount)
            .map(|_| rng.gen_range(self.lowest_game_value..=self.highest_game_value))
            .collect::<GameValues>();
    }
}

impl Default for GameWindow {
    fn default() -> Self {
        
        GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            DEFAULT_LOWEST_GAME_VALUE,
            DEFAULT_SPECIAL_VALUE_INDEX,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .expect("I set these default constants with the constraints in mind.")
    }
}

#[cfg(test)]
pub mod columns_window_tests {
    use super::*;

    #[test]
    fn new_highest_game_value() {
        let game_window = GameWindow::new(
            MIN_HIGHEST_GAME_VALUE - 1,
            DEFAULT_LOWEST_GAME_VALUE,
            DEFAULT_SPECIAL_VALUE_INDEX,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::HighestGameValueTooLow)
        );

        let game_window = GameWindow::new(
            MAX_HIGHEST_GAME_VALUE + 1,
            DEFAULT_LOWEST_GAME_VALUE,
            DEFAULT_SPECIAL_VALUE_INDEX,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::HighestGameValueTooHigh)
        );
    }

    #[test]
    fn new_lowest_game_value() {
        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            MIN_LOWEST_GAME_VALUE - 1,
            DEFAULT_SPECIAL_VALUE_INDEX,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::LowestGameValueTooLow)
        );

        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            MAX_LOWEST_GAME_VALUE + 1,
            DEFAULT_SPECIAL_VALUE_INDEX,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::LowestGameValueTooHigh)
        );
    }

    #[test]
    fn new_special_value_index() {
        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            DEFAULT_LOWEST_GAME_VALUE,
            MIN_SPECIAL_VALUE_INDEX - 1,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::SpecialValueIndexTooLow)
        );

        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            DEFAULT_LOWEST_GAME_VALUE,
            MAX_SPECIAL_VALUE_INDEX + 1,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::SpecialValueIndexTooHigh)
        );

        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            DEFAULT_LOWEST_GAME_VALUE,
            DEFAULT_GAME_VALUE_AMOUNT,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::SpecialValueIndexTooHigh)
        );
    }

    #[test]
    fn new_window_height() {
        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            DEFAULT_LOWEST_GAME_VALUE,
            DEFAULT_SPECIAL_VALUE_INDEX,
            MIN_WINDOW_HEIGHT - 1,
            vec![DEFAULT_LOWEST_GAME_VALUE; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::WindowHeightTooLow)
        );

        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            DEFAULT_LOWEST_GAME_VALUE,
            DEFAULT_SPECIAL_VALUE_INDEX,
            MAX_WINDOW_HEIGHT + 1,
            vec![DEFAULT_LOWEST_GAME_VALUE; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::WindowHeightTooHigh)
        );
    }

    #[test]
    fn new_current_game_values() {
        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            DEFAULT_LOWEST_GAME_VALUE,
            DEFAULT_SPECIAL_VALUE_INDEX,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE - 1; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::SomeCurrentGameValuesTooLow)
        );

        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            DEFAULT_LOWEST_GAME_VALUE,
            DEFAULT_SPECIAL_VALUE_INDEX,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_HIGHEST_GAME_VALUE + 1; DEFAULT_GAME_VALUE_AMOUNT as usize],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::SomeCurrentGameValuesTooHigh)
        );

        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            DEFAULT_LOWEST_GAME_VALUE,
            DEFAULT_SPECIAL_VALUE_INDEX,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE; MIN_GAME_VALUE_AMOUNT as usize - 1],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::TooFewGameValues)
        );

        let game_window = GameWindow::new(
            DEFAULT_HIGHEST_GAME_VALUE,
            DEFAULT_LOWEST_GAME_VALUE,
            DEFAULT_SPECIAL_VALUE_INDEX,
            DEFAULT_WINDOW_HEIGHT,
            vec![DEFAULT_LOWEST_GAME_VALUE; MAX_GAME_VALUE_AMOUNT as usize + 1],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::TooManyGameValues)
        );
    }

    #[test]
    fn init() {
        let mut game_window = GameWindow::default();
        game_window.randomise_game_values();
        assert!(!game_window.current_game_values.iter().all(|game_value| game_value == &DEFAULT_LOWEST_GAME_VALUE));
    }

    #[test]
    fn correct_game_value() {
        let game_window = GameWindow::default();
        assert_eq!(
            game_window.correct_game_value(DEFAULT_LOWEST_GAME_VALUE),
            DEFAULT_LOWEST_GAME_VALUE
        );
        assert_eq!(
            game_window.correct_game_value(DEFAULT_HIGHEST_GAME_VALUE),
            DEFAULT_HIGHEST_GAME_VALUE
        );
        assert_eq!(
            game_window.correct_game_value(DEFAULT_HIGHEST_GAME_VALUE + 1),
            DEFAULT_LOWEST_GAME_VALUE
        );
        assert_eq!(
            game_window.correct_game_value(DEFAULT_LOWEST_GAME_VALUE - 1),
            DEFAULT_HIGHEST_GAME_VALUE
        );
    }

    #[test]
    fn view() {
        let game_window = GameWindow::default();
        let view = game_window.view();
        assert_eq!(view.len() as i8, DEFAULT_WINDOW_HEIGHT);
        let half_window_size = (game_window.window_height / 2) as usize;
        assert_eq!(view[half_window_size], game_window.current_game_values);
        assert_eq!(view[half_window_size + half_window_size][0], game_window.correct_game_value(game_window.current_game_values[0] + half_window_size as i8));
        assert_eq!(view[half_window_size - half_window_size][0], game_window.correct_game_value(game_window.current_game_values[0] - half_window_size as i8));
    }

    #[test]
    fn validate_swipe_parameters() {
        let mut game_window = GameWindow::default();
        assert_eq!(game_window.validate_swipe_parameters(MIN_GAME_VALUE_AMOUNT as i32 - 1, MIN_MOVE_AMOUNT as i32).unwrap_err(), GameWindowError::SwipeError(SwipeError::IndexTooLow));
        assert_eq!(game_window.validate_swipe_parameters(DEFAULT_GAME_VALUE_AMOUNT as i32 + 1, MIN_MOVE_AMOUNT as i32).unwrap_err(), GameWindowError::SwipeError(SwipeError::IndexTooHigh));
        assert_eq!(game_window.validate_swipe_parameters(DEFAULT_GAME_VALUE_AMOUNT as i32, MIN_MOVE_AMOUNT as i32).unwrap_err(), GameWindowError::SwipeError(SwipeError::IndexTooHigh));
        assert_eq!(game_window.validate_swipe_parameters(MIN_GAME_VALUE_AMOUNT as i32, MIN_MOVE_AMOUNT as i32 - 1).unwrap_err(), GameWindowError::SwipeError(SwipeError::MoveAmountTooLow));
        assert_eq!(game_window.validate_swipe_parameters(MIN_GAME_VALUE_AMOUNT as i32, MAX_MOVE_AMOUNT as i32 + 1).unwrap_err(), GameWindowError::SwipeError(SwipeError::MoveAmountTooHigh));
    }

    #[test]
    fn swipe_up() {
        let mut game_window = GameWindow::default();
        let view = game_window.view();
        game_window.swipe_up(DEFAULT_SPECIAL_VALUE_INDEX as i32, DEFAULT_MOVE_AMOUNT as i32).unwrap();
        assert_eq!(view[0][DEFAULT_SPECIAL_VALUE_INDEX as usize], game_window.correct_game_value(game_window.view()[0][DEFAULT_SPECIAL_VALUE_INDEX as usize] - DEFAULT_MOVE_AMOUNT));
        let view = &game_window.view();
        game_window.swipe_up(DEFAULT_SPECIAL_VALUE_INDEX as i32 - 1, DEFAULT_MOVE_AMOUNT as i32).unwrap();
        debug_assert_eq!(view[0][0..=DEFAULT_SPECIAL_VALUE_INDEX as usize - 1], game_window.view()[0][0..=DEFAULT_SPECIAL_VALUE_INDEX as usize - 1].iter().map(|game_value| game_window.correct_game_value(game_value - DEFAULT_MOVE_AMOUNT)).collect::<GameValues>());
        game_window.swipe_up(DEFAULT_SPECIAL_VALUE_INDEX as i32 + 1, DEFAULT_MOVE_AMOUNT as i32).unwrap();
        debug_assert_eq!(view[0][DEFAULT_SPECIAL_VALUE_INDEX as usize + 1..game_window.game_value_amount as usize], game_window.view()[0][DEFAULT_SPECIAL_VALUE_INDEX as usize + 1..game_window.game_value_amount as usize].iter().map(|game_value| game_window.correct_game_value(game_value - DEFAULT_MOVE_AMOUNT)).collect::<GameValues>());
    }  

    #[test]
    fn swipe_down() {
        let mut game_window = GameWindow::default();
        let view = game_window.view();
        game_window.swipe_down(DEFAULT_SPECIAL_VALUE_INDEX as i32, DEFAULT_MOVE_AMOUNT as i32).unwrap();
        assert_eq!(view[0][DEFAULT_SPECIAL_VALUE_INDEX as usize], game_window.correct_game_value(game_window.view()[0][DEFAULT_SPECIAL_VALUE_INDEX as usize] + DEFAULT_MOVE_AMOUNT));
        let view = &game_window.view();
        game_window.swipe_down(DEFAULT_SPECIAL_VALUE_INDEX as i32 - 1, DEFAULT_MOVE_AMOUNT as i32).unwrap();
        debug_assert_eq!(view[0][0..=DEFAULT_SPECIAL_VALUE_INDEX as usize - 1], game_window.view()[0][0..=DEFAULT_SPECIAL_VALUE_INDEX as usize - 1].iter().map(|game_value| game_window.correct_game_value(game_value + DEFAULT_MOVE_AMOUNT)).collect::<GameValues>());
        let view = &game_window.view();
        game_window.swipe_down(DEFAULT_SPECIAL_VALUE_INDEX as i32 + 1, DEFAULT_MOVE_AMOUNT as i32).unwrap();
        debug_assert_eq!(view[0][DEFAULT_SPECIAL_VALUE_INDEX as usize + 1..game_window.game_value_amount as usize], game_window.view()[0][DEFAULT_SPECIAL_VALUE_INDEX as usize + 1..game_window.game_value_amount as usize].iter().map(|game_value| game_window.correct_game_value(game_value + DEFAULT_MOVE_AMOUNT)).collect::<GameValues>());
    }   
}
