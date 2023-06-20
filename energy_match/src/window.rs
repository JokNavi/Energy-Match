use rand::prelude::*;

type Column = Vec<u8>;
type Columns = Vec<Column>;
type ColumnsSelectedIndexes = Vec<u8>;
type Window<'a> = Vec<&'a [u8]>;

const DEFAULT_COLUMN_AMOUNT: u8 = 11;
const DEFAULT_COLUMN_HEIGHT: u8 = 8;
const DEFAULT_WINDOW_HEIGHT: u8 = 3;
const DEFAULT_ROOT_VALUE_INDEX: u8 = 6;

pub const MIN_COLUMN_AMOUNT: u8 = 1;
pub const MIN_COLUMN_HEIGHT: u8 = 2;
pub const MIN_WINDOW_HEIGHT: u8 = 1;

pub const MAX_COLUMN_AMOUNT: u8 = 99;
pub const MAX_COLUMN_HEIGHT: u8 = 99;
pub const MAX_WINDOW_HEIGHT: u8 = 99;
pub const MAX_ROOT_VALUE_INDEX: u8 = MAX_COLUMN_AMOUNT - 1;

#[derive(PartialEq, Debug)]
pub enum NewColumnsWindowError {
    NotEnoughColumnValues, // InvalidColumnAmount
    TooManyColumnValues,   // InvalidColumnAmount
    NotEnoughColumns,      // InvalidColumnHeight
    TooManyColumns,        // InvalidColumnHeight
    WindowTooSmall,        // InvalidWindowHeight
    WindowTooBig,          // InvalidWindowHeight
    RootValueIndexTooBig,  // InvalidRootValueIndex
}

#[derive(Debug)]
pub struct ColumnsWindow {
    columns: Columns,
    column_amount: u8,
    column_height: u8,
    window_height: u8,
    root_value_index: u8,
    columns_selected_indexes: ColumnsSelectedIndexes,
}

impl ColumnsWindow {
    pub fn new(
        column_amount: u8,
        column_height: u8,
        window_height: u8,
        root_value_index: u8,
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

        match column_height {
            _ if column_height < MIN_COLUMN_HEIGHT => {
                return Err(NewColumnsWindowError::NotEnoughColumnValues)
            }
            _ if column_height > MAX_COLUMN_HEIGHT => {
                return Err(NewColumnsWindowError::TooManyColumnValues)
            }
            _ => (),
        };

        match window_height {
            _ if window_height < MIN_WINDOW_HEIGHT => {
                return Err(NewColumnsWindowError::WindowTooSmall)
            }
            _ if window_height > MAX_WINDOW_HEIGHT => {
                return Err(NewColumnsWindowError::WindowTooBig)
            }
            _ => (),
        };

        match root_value_index {
            _ if root_value_index > MAX_ROOT_VALUE_INDEX => {
                return Err(NewColumnsWindowError::RootValueIndexTooBig)
            }
            _ => (),
        };

        let columns: Columns =
            vec![(MIN_COLUMN_AMOUNT..=window_height).collect(); column_amount as usize];
        let mut rng = thread_rng();
        let columns_selected_indexes: ColumnsSelectedIndexes =
            vec![rng.gen_range(1..=column_height); column_amount as usize]; //random for now
        Ok(Self {
            columns,
            column_amount,
            column_height,
            window_height,
            root_value_index,
            columns_selected_indexes,
        })
    }

    pub fn adjust_column_index(&self, value: i8) -> u8 {
        (value % self.column_height as i8) as u8
    }

    pub fn window(&self) -> Window {
        let half_window_size = (self.window_height as f32 / 2 as f32).round() as u8;
        let mut window = Vec::with_capacity(self.column_amount as usize);
        for (i, column) in self.columns.iter().enumerate() {
            let top_column_index = self.adjust_column_index(
                self.columns_selected_indexes[i] as i8 + half_window_size as i8,
            );

            let bottom_column_index = self.adjust_column_index(
                self.columns_selected_indexes[i] as i8 - half_window_size as i8,
            );
            window.push(&column[bottom_column_index as usize..=top_column_index as usize])
        }
        window
    }
}

impl Default for ColumnsWindow {
    fn default() -> Self {
        ColumnsWindow::new(
            DEFAULT_COLUMN_AMOUNT,
            DEFAULT_COLUMN_HEIGHT,
            DEFAULT_WINDOW_HEIGHT,
            DEFAULT_ROOT_VALUE_INDEX,
        )
        .expect("I set these default values with the constraints in mind.")
    }
}

impl PartialEq for ColumnsWindow {
    fn eq(&self, other: &Self) -> bool {
        self.column_amount == other.column_amount
            && self.column_height == other.column_height
            && self.window_height == other.window_height
            && self.root_value_index == other.root_value_index
            && self.columns_selected_indexes == other.columns_selected_indexes
            //doesn't compare self.columns
    }
}

#[cfg(test)]
pub mod columns_window_tests {
    use super::*;

    #[test]
    fn new_minimum_values() {
        let columns_window = ColumnsWindow::new(
            MIN_COLUMN_AMOUNT - 1,
            MIN_COLUMN_HEIGHT,
            MIN_WINDOW_HEIGHT,
            0,
        );
        assert!(columns_window.is_err());
        assert_eq!(columns_window, Err(NewColumnsWindowError::NotEnoughColumns));

        let columns_window = ColumnsWindow::new(
            MIN_COLUMN_AMOUNT,
            MIN_COLUMN_HEIGHT - 1,
            MIN_WINDOW_HEIGHT,
            0,
        );
        assert!(columns_window.is_err());
        assert_eq!(
            columns_window,
            Err(NewColumnsWindowError::NotEnoughColumnValues)
        );

        let columns_window = ColumnsWindow::new(
            MIN_COLUMN_AMOUNT,
            MIN_COLUMN_HEIGHT,
            MIN_WINDOW_HEIGHT - 1,
            0,
        );
        assert!(columns_window.is_err());
        assert_eq!(columns_window, Err(NewColumnsWindowError::WindowTooSmall));

        let columns_window =
            ColumnsWindow::new(MIN_COLUMN_AMOUNT, MIN_COLUMN_HEIGHT, MIN_WINDOW_HEIGHT, 0);
        assert!(columns_window.is_ok());
    }

    #[test]
    fn new_maxiumum_values() {
        let columns_window = ColumnsWindow::new(
            MAX_COLUMN_AMOUNT + 1,
            MAX_COLUMN_HEIGHT,
            MAX_WINDOW_HEIGHT,
            0,
        );
        assert!(columns_window.is_err());
        assert_eq!(columns_window, Err(NewColumnsWindowError::TooManyColumns));

        let columns_window = ColumnsWindow::new(
            MAX_COLUMN_AMOUNT,
            MAX_COLUMN_HEIGHT + 1,
            MAX_WINDOW_HEIGHT,
            0,
        );
        assert!(columns_window.is_err());
        assert_eq!(
            columns_window,
            Err(NewColumnsWindowError::TooManyColumnValues)
        );

        let columns_window = ColumnsWindow::new(
            MAX_COLUMN_AMOUNT,
            MAX_COLUMN_HEIGHT,
            MAX_WINDOW_HEIGHT + 1,
            0,
        );
        assert!(columns_window.is_err());
        assert_eq!(columns_window, Err(NewColumnsWindowError::WindowTooBig));

        let columns_window =
            ColumnsWindow::new(MAX_COLUMN_AMOUNT, MAX_COLUMN_HEIGHT, MAX_WINDOW_HEIGHT, 0);
        assert!(columns_window.is_ok());
    }

    #[test]
    fn default() {
        let default_columns_window = ColumnsWindow::new(
            DEFAULT_COLUMN_AMOUNT,
            DEFAULT_COLUMN_HEIGHT,
            DEFAULT_WINDOW_HEIGHT,
            DEFAULT_ROOT_VALUE_INDEX,
        );
        assert!(default_columns_window.is_ok());
        assert_eq!(default_columns_window.unwrap(), ColumnsWindow::default());
    }
}
