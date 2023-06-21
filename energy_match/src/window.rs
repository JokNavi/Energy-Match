use rand::prelude::*;

type Column = Vec<u8>;
type Columns = Vec<Column>;
type ColumnsSelectedIndexes = Vec<u8>;
type Window<'a> = Vec<WindowRow<'a>>;
type WindowRow<'a> = Vec<&'a u8>;

const DEFAULT_COLUMN_AMOUNT: u8 = 11;
const DEFAULT_COLUMN_LENGTH: u8 = 8;
const DEFAULT_WINDOW_HEIGHT: u8 = 3;
const DEFAULT_ROOT_VALUE_INDEX: u8 = 6;
const DEFAULT_ENERGY: i32 = (DEFAULT_COLUMN_AMOUNT * DEFAULT_COLUMN_LENGTH) as i32;

pub const MIN_COLUMN_AMOUNT: u8 = 1;
pub const MIN_COLUMN_LENGTH: u8 = 2;
pub const MIN_WINDOW_HEIGHT: u8 = 1;

pub const MAX_COLUMN_AMOUNT: u8 = 99;
pub const MAX_COLUMN_LENGTH: u8 = 99;
pub const MAX_WINDOW_HEIGHT: u8 = MAX_COLUMN_LENGTH / 2;
pub const MAX_ROOT_VALUE_INDEX: u8 = MAX_COLUMN_AMOUNT - 1;

#[derive(PartialEq, Debug)]
pub enum NewColumnsWindowError {
    NotEnoughColumnValues, // InvalidColumnAmount
    TooManyColumnValues,   // InvalidColumnAmount
    NotEnoughColumns,      // InvalidColumnHeight
    TooManyColumns,        // InvalidColumnHeight
    WindowHeightTooSmall,  // InvalidWindowHeight
    WindowHeightTooLarge,  // InvalidWindowHeight
    RootValueIndexTooBig,  // InvalidRootValueIndex
}

#[derive(PartialEq, Debug)]
pub enum SwipeError {
    IndexTooBig,
    IndexTooSmall,
    AmountTooMany,
    AmountTooFew,
    NotEnoughEnergy,
}

pub fn order_indexes(index_1: u8, index_2: u8) -> (usize, usize) {
    if index_1 < index_2 {
        return (index_1.into(), index_2.into());
    }
    (index_2.into(), index_1.into())
}

#[derive(Debug, Clone)]
pub struct ColumnsWindow {
    columns: Columns,
    column_amount: u8,
    column_length: u8,
    window_height: u8,
    root_value_index: u8,
    columns_selected_indexes: ColumnsSelectedIndexes,
    energy: i32,
}

impl ColumnsWindow {
    pub fn new(
        column_amount: u8,
        column_length: u8,
        window_height: u8,
        root_value_index: u8,
        energy: i32,
    ) -> Result<Self, NewColumnsWindowError> {
        match column_amount {
            _ if column_amount < MIN_COLUMN_AMOUNT => {
                return Err(NewColumnsWindowError::NotEnoughColumns)
            }
            _ if column_amount > MAX_COLUMN_AMOUNT => {
                return Err(NewColumnsWindowError::TooManyColumns)
            }
            _ => (),
        };

        match column_length {
            _ if column_length < MIN_COLUMN_LENGTH => {
                return Err(NewColumnsWindowError::NotEnoughColumnValues)
            }
            _ if column_length > MAX_COLUMN_LENGTH => {
                return Err(NewColumnsWindowError::TooManyColumnValues)
            }
            _ => (),
        };

        match window_height {
            _ if window_height < MIN_WINDOW_HEIGHT => {
                return Err(NewColumnsWindowError::WindowHeightTooSmall)
            }
            _ if window_height > MAX_WINDOW_HEIGHT => {
                return Err(NewColumnsWindowError::WindowHeightTooLarge)
            }
            _ => (),
        };

        match root_value_index {
            _ if root_value_index > MAX_ROOT_VALUE_INDEX => {
                return Err(NewColumnsWindowError::RootValueIndexTooBig)
            }
            _ => (),
        };

        let columns: Columns = vec![
            (MIN_COLUMN_AMOUNT..=column_length)
                .collect::<Column>()
                .clone();
            column_amount as usize
        ];
        let mut rng = thread_rng();
        let columns_selected_indexes = (0..column_amount)
            .into_iter()
            .map(|_| rng.gen_range(0..column_length))
            .collect::<ColumnsSelectedIndexes>(); //random for now
        Ok(Self {
            columns,
            column_amount,
            column_length,
            window_height,
            root_value_index,
            columns_selected_indexes,
            energy,
        })
    }

    pub fn correct_column_length(&self, value: i8) -> u8 {
        let column_length = (self.column_length - MIN_COLUMN_LENGTH + 1) as i8;
        let index = (value - MIN_COLUMN_LENGTH as i8) % column_length;
        println!("{} % {} = {}", value, column_length, index);
        if index < 0 {
            return (self.column_length as i8 + index) as u8;
        }
        return (MIN_COLUMN_LENGTH as i8 + index) as u8;
    }

    pub fn window(&self) -> Window {
        let half_window_size = self.window_height as i8 / 2;
        let mut window: Window = Vec::with_capacity(self.window_height as usize);
        for window_index_ajustment in (-half_window_size)..=half_window_size {
            let mut window_row: WindowRow = Vec::with_capacity(self.column_amount as usize);
            for (i, column) in self.columns.iter().enumerate() {
                let selected_index = self.correct_column_length(
                    self.columns_selected_indexes[i] as i8 + window_index_ajustment,
                );
                window_row.push(&column[selected_index as usize]);
            }
            window.push(window_row);
        }
        window
    }

    //both index parameters start from 1.
    pub fn swipe_up(&mut self, index: i32, amount: i32) -> Result<(), SwipeError> {
        let index = index - 1;
        match index {
            _ if index < MIN_COLUMN_AMOUNT.into() => return Err(SwipeError::IndexTooSmall),
            _ if index >= (self.column_amount -MIN_COLUMN_AMOUNT) as i32 => return Err(SwipeError::IndexTooBig),
            _ => (),
        };
        match amount {
            _ if amount < 1 => return Err(SwipeError::AmountTooFew),
            _ if amount >= self.column_length.into() => return Err(SwipeError::AmountTooMany),
            _ if amount > self.energy => return Err(SwipeError:: NotEnoughEnergy),
            _ => (),
        };

        let (smaller_index, bigger_index) = order_indexes(index as u8, self.root_value_index);
        (smaller_index..=bigger_index).for_each(|i| {
            self.columns_selected_indexes[i] =
                self.correct_column_length(self.columns_selected_indexes[i] as i8 - amount as i8)
        });
        self.energy = self.energy - amount;
        Ok(())
    }
}

impl Default for ColumnsWindow {
    fn default() -> Self {
        ColumnsWindow::new(
            DEFAULT_COLUMN_AMOUNT,
            DEFAULT_COLUMN_LENGTH,
            DEFAULT_WINDOW_HEIGHT,
            DEFAULT_ROOT_VALUE_INDEX,
            DEFAULT_ENERGY,
        )
        .expect("I set these default values with the constraints in mind.")
    }
}

impl PartialEq for ColumnsWindow {
    fn eq(&self, other: &Self) -> bool {
        self.column_amount == other.column_amount
            && self.column_length == other.column_length
            && self.window_height == other.window_height
            && self.root_value_index == other.root_value_index
            // && self.columns_selected_indexes == other.columns_selected_indexes 
            && self.columns == other.columns
    }
}

#[cfg(test)]
pub mod columns_window_tests {
    use super::*;

    #[test]
    fn new_minimum_values() {
        let columns_window = ColumnsWindow::new(
            MIN_COLUMN_AMOUNT - 1,
            MIN_COLUMN_LENGTH,
            MIN_WINDOW_HEIGHT,
            0,
            DEFAULT_ENERGY,
        );
        assert!(columns_window.is_err());
        assert_eq!(columns_window, Err(NewColumnsWindowError::NotEnoughColumns));

        let columns_window = ColumnsWindow::new(
            MIN_COLUMN_AMOUNT,
            MIN_COLUMN_LENGTH - 1,
            MIN_WINDOW_HEIGHT,
            0,
            DEFAULT_ENERGY,
        );
        assert!(columns_window.is_err());
        assert_eq!(
            columns_window,
            Err(NewColumnsWindowError::NotEnoughColumnValues)
        );

        let columns_window = ColumnsWindow::new(
            MIN_COLUMN_AMOUNT,
            MIN_COLUMN_LENGTH,
            MIN_WINDOW_HEIGHT - 1,
            0,
            DEFAULT_ENERGY,
        );
        assert!(columns_window.is_err());
        assert_eq!(
            columns_window,
            Err(NewColumnsWindowError::WindowHeightTooSmall)
        );

        let columns_window = ColumnsWindow::new(
            MIN_COLUMN_AMOUNT,
            MIN_COLUMN_LENGTH,
            MIN_WINDOW_HEIGHT,
            0,
            DEFAULT_ENERGY,
        );
        assert!(columns_window.is_ok());
    }

    #[test]
    fn new_maxiumum_values() {
        let columns_window = ColumnsWindow::new(
            MAX_COLUMN_AMOUNT + 1,
            MAX_COLUMN_LENGTH,
            MAX_WINDOW_HEIGHT,
            0,
            DEFAULT_ENERGY,
        );
        assert!(columns_window.is_err());
        assert_eq!(columns_window, Err(NewColumnsWindowError::TooManyColumns));

        let columns_window = ColumnsWindow::new(
            MAX_COLUMN_AMOUNT,
            MAX_COLUMN_LENGTH + 1,
            MAX_WINDOW_HEIGHT,
            0,
            DEFAULT_ENERGY,
        );
        assert!(columns_window.is_err());
        assert_eq!(
            columns_window,
            Err(NewColumnsWindowError::TooManyColumnValues)
        );

        let columns_window = ColumnsWindow::new(
            MAX_COLUMN_AMOUNT,
            MAX_COLUMN_LENGTH,
            MAX_WINDOW_HEIGHT + 1,
            0,
            DEFAULT_ENERGY,
        );
        assert!(columns_window.is_err());
        assert_eq!(
            columns_window,
            Err(NewColumnsWindowError::WindowHeightTooLarge)
        );

        let columns_window = ColumnsWindow::new(
            MAX_COLUMN_AMOUNT,
            MAX_COLUMN_LENGTH,
            MAX_WINDOW_HEIGHT,
            0,
            DEFAULT_ENERGY,
        );
        assert!(columns_window.is_ok());
    }

    #[test]
    fn default() {
        let default_columns_window = ColumnsWindow::new(
            DEFAULT_COLUMN_AMOUNT,
            DEFAULT_COLUMN_LENGTH,
            DEFAULT_WINDOW_HEIGHT,
            DEFAULT_ROOT_VALUE_INDEX,
            DEFAULT_ENERGY,
        );
        assert!(default_columns_window.is_ok());
        assert_eq!(default_columns_window.unwrap(), ColumnsWindow::default());
    }

    #[test]
    fn correct_column_index() {
        let columns_window = ColumnsWindow::default();
        assert_eq!(columns_window.correct_column_length(DEFAULT_COLUMN_LENGTH as i8), DEFAULT_COLUMN_LENGTH); //max value
        assert_eq!(columns_window.correct_column_length(DEFAULT_COLUMN_LENGTH as i8 + 1), MIN_COLUMN_LENGTH);
        assert_eq!(columns_window.correct_column_length(DEFAULT_COLUMN_LENGTH as i8*2), MIN_COLUMN_LENGTH);
        assert_eq!(columns_window.correct_column_length(MIN_COLUMN_LENGTH as i8), MIN_COLUMN_LENGTH);
    }

    fn check_window_offset(columns_window: &ColumnsWindow, row_one: &WindowRow, row_two: &WindowRow, offset: u8) -> bool {
        let results: Vec<bool> = row_one.iter().enumerate().map(|(i, row_one_value)| {let answer =  columns_window.correct_column_length((**row_one_value + offset) as i8); println!("{} == {}", answer, row_two[i]); *row_two[i] == answer}).collect();
        println!("row_one: {:?}", row_one);
        println!("row_two: {:?}", row_two);
        !results.contains(&false)
        }

    #[test]
    fn window() {
        let columns_window = ColumnsWindow::new(
            DEFAULT_COLUMN_AMOUNT,
            DEFAULT_COLUMN_LENGTH,
            3,
            DEFAULT_ROOT_VALUE_INDEX,
            DEFAULT_ENERGY,
        )
        .unwrap();
        let window = columns_window.window();
        assert_eq!(window.len() as u8, 3);
        assert!(check_window_offset(&columns_window, &window[1], &window[2], 1));
    }

    #[test]
    fn swipe_up_index() {
        let mut columns_window = ColumnsWindow::default();
        assert_eq!(columns_window.swipe_up(0, 1), Err(SwipeError::IndexTooSmall));
        assert_eq!(columns_window.swipe_up(DEFAULT_COLUMN_AMOUNT.into(), 1), Err(SwipeError::IndexTooBig));
        assert!(columns_window.swipe_up((DEFAULT_COLUMN_AMOUNT - MIN_COLUMN_AMOUNT) as i32 , 1).is_ok());
        assert!(columns_window.swipe_up(MIN_COLUMN_AMOUNT as i32 + 1, 1).is_ok());
    }

    #[test]
    fn swipe_up_amount() {
        let mut columns_window = ColumnsWindow::default();
        assert_eq!(columns_window.swipe_up(MIN_COLUMN_AMOUNT as i32 + 1, 0), Err(SwipeError::AmountTooFew));
        assert_eq!(columns_window.swipe_up(MIN_COLUMN_AMOUNT as i32 + 1, DEFAULT_ENERGY+1), Err(SwipeError::AmountTooMany));
        assert_eq!(columns_window.swipe_up(MIN_COLUMN_AMOUNT as i32 + 1, DEFAULT_COLUMN_AMOUNT as i32 + 1), Err(SwipeError::AmountTooMany));
    }

    #[test]
    fn swipe_up_window() {
        let mut columns_window = ColumnsWindow::default();
        let first_window: Vec<Vec<&u8>> = columns_window.window();
        assert!(columns_window.clone().swipe_up(MIN_COLUMN_AMOUNT as i32 + 1, 1).is_ok());
        let second_window = columns_window.window();
        assert!(check_window_offset(&columns_window, &first_window[1], &second_window[2], 1));
    }
}
