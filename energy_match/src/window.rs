use rand::prelude::*;

pub type Column = Vec<u8>;
pub type Columns = Vec<Column>;
pub type ColumnsSelectedIndexes = Vec<u8>;
pub type Window<'a> = Vec<&'a [u8]>;

const DEFAULT_COLUMN_AMOUNT: u8 = 11;
const DEFAULT_COLUMN_HEIGHT: u8 = 8;
const DEFAULT_WINDOW_HEIGHT: u8 = 3;
const DEFAULT_ROOT_VALUE_INDEX: u8 = 6;

#[derive(Debug)]
pub enum NewColumnWindowError {}

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
    ) -> Result<Self, NewColumnWindowError> {
        let columns: Columns = vec![(1..=window_height).collect(); column_amount as usize];
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
