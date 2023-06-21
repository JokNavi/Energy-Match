use rand::prelude::*;

type Column = Vec<u8>;
type Columns = Vec<Column>;
type ColumnsSelectedIndexes = Vec<u8>;
type Window<'a> = Vec<WindowRow<'a>>;
type WindowRow<'a> = Vec<&'a u8>;
type ColumnSliceBoundaries = (u8, u8, u8);

const DEFAULT_COLUMN_AMOUNT: u8 = 11;
const DEFAULT_COLUMN_LENGTH: u8 = 8;
const DEFAULT_WINDOW_HEIGHT: u8 = 3;
const DEFAULT_ROOT_VALUE_INDEX: u8 = 6;

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
    WindowTooSmall,        // InvalidWindowHeight
    WindowTooBig,          // InvalidWindowHeight
    RootValueIndexTooBig,  // InvalidRootValueIndex
}

pub fn order_indexes(index_1: usize, index_2: usize) -> (usize, usize) {
    if index_1 < index_2 {
        return (index_1, index_2);
    }
    (index_2, index_1)
}

#[derive(Debug)]
pub struct ColumnsWindow {
    columns: Columns,
    column_amount: u8,
    column_length: u8,
    window_height: u8,
    root_value_index: u8,
    columns_selected_indexes: ColumnsSelectedIndexes,
}

impl ColumnsWindow {
    pub fn new(
        column_amount: u8,
        column_length: u8,
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
        })
    }

    pub fn correct_column_length(&self, value: i8) -> u8 {
        let column_length = (self.column_length - MIN_COLUMN_AMOUNT) as i8;
        let index = value % column_length;
        if index < 0 {
            return (self.column_length as i8 - 1 + index) as u8;
        }
        return index as u8;
    }

    pub fn window(&self) -> Window {
        let half_window_size = self.window_height as i8 / 2;
        let mut window: Window = Vec::with_capacity(self.window_height as usize);
        for window_index_ajustment in (-half_window_size)..=half_window_size {
            let mut window_row: WindowRow = Vec::with_capacity(self.column_amount as usize);
            for (i, column) in self.columns.iter().enumerate() {
                let selected_index = self.correct_column_length(self.columns_selected_indexes[i] as i8 + window_index_ajustment);
                window_row.push(&column[selected_index as usize]);
            }
            window.push(window_row);
        }
        window
    }
}

impl Default for ColumnsWindow {
    fn default() -> Self {
        ColumnsWindow::new(
            DEFAULT_COLUMN_AMOUNT,
            DEFAULT_COLUMN_LENGTH,
            DEFAULT_WINDOW_HEIGHT,
            DEFAULT_ROOT_VALUE_INDEX,
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
        );
        assert!(columns_window.is_err());
        assert_eq!(columns_window, Err(NewColumnsWindowError::NotEnoughColumns));

        let columns_window = ColumnsWindow::new(
            MIN_COLUMN_AMOUNT,
            MIN_COLUMN_LENGTH - 1,
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
            MIN_COLUMN_LENGTH,
            MIN_WINDOW_HEIGHT - 1,
            0,
        );
        assert!(columns_window.is_err());
        assert_eq!(columns_window, Err(NewColumnsWindowError::WindowTooSmall));

        let columns_window =
            ColumnsWindow::new(MIN_COLUMN_AMOUNT, MIN_COLUMN_LENGTH, MIN_WINDOW_HEIGHT, 0);
        assert!(columns_window.is_ok());
    }

    #[test]
    fn new_maxiumum_values() {
        let columns_window = ColumnsWindow::new(
            MAX_COLUMN_AMOUNT + 1,
            MAX_COLUMN_LENGTH,
            MAX_WINDOW_HEIGHT,
            0,
        );
        assert!(columns_window.is_err());
        assert_eq!(columns_window, Err(NewColumnsWindowError::TooManyColumns));

        let columns_window = ColumnsWindow::new(
            MAX_COLUMN_AMOUNT,
            MAX_COLUMN_LENGTH + 1,
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
            MAX_COLUMN_LENGTH,
            MAX_WINDOW_HEIGHT + 1,
            0,
        );
        assert!(columns_window.is_err());
        assert_eq!(columns_window, Err(NewColumnsWindowError::WindowTooBig));

        let columns_window =
            ColumnsWindow::new(MAX_COLUMN_AMOUNT, MAX_COLUMN_LENGTH, MAX_WINDOW_HEIGHT, 0);
        assert!(columns_window.is_ok());
    }

    #[test]
    fn default() {
        let default_columns_window = ColumnsWindow::new(
            DEFAULT_COLUMN_AMOUNT,
            DEFAULT_COLUMN_LENGTH,
            DEFAULT_WINDOW_HEIGHT,
            DEFAULT_ROOT_VALUE_INDEX,
        );
        assert!(default_columns_window.is_ok());
        assert_eq!(default_columns_window.unwrap(), ColumnsWindow::default());
    }

    #[test]
    fn correct_column_index() {
        let columns_window = ColumnsWindow::default();
        let column_length = (DEFAULT_COLUMN_LENGTH as i8) - 1;
        assert_eq!(
            columns_window.correct_column_length(column_length - 1),
            (column_length - 1) as u8
        );
        assert_eq!(columns_window.correct_column_length(column_length), 0);
        assert_eq!(columns_window.correct_column_length(column_length + 2), 2);
        assert_eq!(
            columns_window.correct_column_length(column_length + column_length),
            0
        );
        assert_eq!(
            columns_window.correct_column_length(-1),
            (column_length - 1) as u8
        );
        assert_eq!(
            columns_window.correct_column_length(-(column_length / 2)),
            4
        );
    }

    #[test]
    fn window() {
        let columns_window = ColumnsWindow::new(
            DEFAULT_COLUMN_AMOUNT,
            DEFAULT_COLUMN_LENGTH,
            3,
            DEFAULT_ROOT_VALUE_INDEX,
        )
        .unwrap();
        let window = columns_window.window();
        assert_eq!(window.len() as u8, 3);
        for (i, middle_window_row_item) in window[1].iter().enumerate() {
            let top_window_row_item = columns_window.correct_column_length((window[2][i]-1) as i8); //crude hack
            let middle_window_row_item = columns_window.correct_column_length(**middle_window_row_item as i8); //crude hack
            assert_eq!(middle_window_row_item, top_window_row_item);
        }
    }
}
