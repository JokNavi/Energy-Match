use rand::prelude::*;

pub type ColumnsSelectedIndexes = Vec<u8>;
pub type Columns = Vec<Vec<u8>>;

const DEFAULT_COLUMN_AMOUNT: u8 = 11;
const DEFAULT_COLUMN_HEIGHT: u8 = 8;
const DEFAULT_WINDOW_HEIGHT: u8 = 3;
const DEFAULT_ROOT_VALUE_INDEX: u8 = 6;

pub struct ColumnWindow {
    columns: Columns,
    column_amount: u8,
    column_height: u8,
    window_height: u8,
    root_value_index: u8,
    columns_selected_indexes: ColumnsSelectedIndexes,
}

impl ColumnWindow {
    pub fn new(
        column_amount: u8,
        column_height: u8,
        window_height: u8,
        root_value_index: u8,

    ) -> Self {
        let columns: Columns = vec![(1..=window_height).collect(); column_amount as usize];
        let mut rng = thread_rng();
        let columns_selected_indexes: ColumnsSelectedIndexes = vec![rng.gen_range(0..column_height); column_amount as usize]; //random for now
        Self {
            columns,
            column_amount,
            column_height,
            window_height,
            root_value_index,
            columns_selected_indexes,
        }
    }
}

impl Default for ColumnWindow {
    fn default() -> Self {
        ColumnWindow::new(
            DEFAULT_COLUMN_AMOUNT,
            DEFAULT_COLUMN_HEIGHT,
            DEFAULT_WINDOW_HEIGHT,
            DEFAULT_ROOT_VALUE_INDEX,
        )
    }
}
