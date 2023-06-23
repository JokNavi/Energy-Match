const MIN_LOWEST_GAME_VALUE: i8 = 0; // can change.
const MIN_HIGHEST_GAME_VALUE: i8 = MIN_LOWEST_GAME_VALUE + 1;
const MIN_SPECIAL_VALUE_INDEX: i8 = 0;
const MIN_WINDOW_HEIGHT: i8 = MIN_HIGHEST_GAME_VALUE;
const MIN_GAME_VALUE_AMOUNT: i8 = 1; // can change

const MAX_HIGHEST_GAME_VALUE: i8 = 99; // can change
const MAX_LOWEST_GAME_VALUE: i8 = MAX_HIGHEST_GAME_VALUE - 1;
const MAX_SPECIAL_VALUE_INDEX: i8 = MAX_HIGHEST_GAME_VALUE;
const MAX_WINDOW_HEIGHT: i8 = MAX_HIGHEST_GAME_VALUE;
const MAX_GAME_VALUE_AMOUNT: i8 = 20; // can change

const DEFAULT_LOWEST_GAME_VALUE: i8 = 1; // can change
const DEFAULT_HIGHEST_GAME_VALUE: i8 = 8; // can change
const DEFAULT_SPECIAL_VALUE_INDEX: i8 = 6;
const DEFAULT_WINDOW_HEIGHT: i8 = 3; // can change
const DEFAULT_GAME_VALUE_AMOUNT: i8 = 11; // can change

pub type CurrentGameValues = Vec<i8>;

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

}

#[derive(Debug, Clone)]
pub struct GameWindow {
    highest_game_value: i8,
    lowest_game_value: i8,
    special_value_index: i8,
    window_height: i8,
    game_value_amount: i8,
    current_game_values: CurrentGameValues,
}

impl GameWindow {
    pub fn new(
        highest_game_value: i8,
        lowest_game_value: i8,
        special_value_index: i8,
        window_height: i8,
        current_game_values: CurrentGameValues,
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
        return self.lowest_game_value + index;
    }

    // pub fn correct_columns_selected_index(&self, value: i8) -> u8 {
    //     let index = value % self.column_height as i8;
    //     println!("{} % {} = {}", value, self.column_height as i8, index);
    //     if index < 0 {
    //         return (self.column_height as i8 + index) as u8;
    //     }
    //     return index as u8;
    // }

    // pub fn window(&self) -> Window {
    //     let half_window_size = self.window_height as i8 / 2;
    //     let mut window: Window = Vec::with_capacity(self.window_height as usize);
    //     for window_index_ajustment in (-half_window_size)..=half_window_size {
    //         let mut window_row: WindowRow = Vec::with_capacity(self.column_amount as usize);
    //         for (i, column) in self.columns.iter().enumerate() {
    //             let selected_index = self.correct_column_values(
    //                 self.columns_selected_indexes[i] as i8 + window_index_ajustment,
    //             );
    //             window_row.push(&column[selected_index as usize]);
    //         }
    //         window.push(window_row);
    //     }
    //     window
    // }

    // //both index parameters start from 1.
    // pub fn swipe_up(&mut self, index: i32, amount: i32) -> Result<(), SwipeError> {
    //     let index = index - 1;
    //     match index {
    //         _ if index < MIN_COLUMN_AMOUNT.into() => return Err(SwipeError::IndexTooSmall),
    //         _ if index >= (self.column_amount - MIN_COLUMN_AMOUNT) as i32 => {
    //             return Err(SwipeError::IndexTooBig)
    //         }
    //         _ => (),
    //     };
    //     match amount {
    //         _ if amount < 1 => return Err(SwipeError::AmountTooFew),
    //         _ if amount >= self.column_height.into() => return Err(SwipeError::AmountTooMany),
    //         _ if amount > self.energy => return Err(SwipeError::NotEnoughEnergy),
    //         _ => (),
    //     };

    //     let (smaller_index, bigger_index) = order_indexes(index as u8, self.root_value_index);
    //     (smaller_index..=bigger_index).for_each(|i| {
    //         self.columns_selected_indexes[i] =
    //             self.correct_column_values(self.columns_selected_indexes[i] as i8 - amount as i8)
    //     });
    //     self.energy = self.energy - amount;
    //     Ok(())
    // }
}

impl Default for GameWindow {
    fn default() -> Self {
        //let mut rng = thread_rng();
        //let columns_selected_indexes = (0..DEFAULT_COLUMN_AMOUNT)
        //    .into_iter()
        //    .map(|_| rng.gen_range(0..DEFAULT_COLUMN_HEIGHT))
        //    .collect::<ColumnsSelectedIndexes>();
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
            vec![DEFAULT_LOWEST_GAME_VALUE; MAX_GAME_VALUE_AMOUNT as usize  + 1 ],
        )
        .unwrap_err();

        assert_eq!(
            game_window,
            GameWindowError::NewGameWindowError(NewGameWindowError::TooManyGameValues)
        );
    }

    #[test] 
    fn correct_game_value(){
        let game_window = GameWindow::default();
        assert_eq!(game_window.correct_game_value(DEFAULT_LOWEST_GAME_VALUE), DEFAULT_LOWEST_GAME_VALUE);
        assert_eq!(game_window.correct_game_value(DEFAULT_HIGHEST_GAME_VALUE), DEFAULT_HIGHEST_GAME_VALUE);
        assert_eq!(game_window.correct_game_value(DEFAULT_HIGHEST_GAME_VALUE+1), DEFAULT_LOWEST_GAME_VALUE);
        assert_eq!(game_window.correct_game_value(DEFAULT_LOWEST_GAME_VALUE-1), DEFAULT_HIGHEST_GAME_VALUE);
    }
}
