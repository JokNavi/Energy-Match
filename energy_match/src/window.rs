const DefaultAmountOfColumns: u8 = 11;
const DefaultColumnHeight: u8 = 8;
const DefaultWindowHeight: u8 = 3;
const DefaultRootValueIndex: u8 = 6;

pub struct ColumnWindow {
    columns: Vec<Vec<u8>>,
    column_amount: u8,
    column_height: u8,
    window_height: u8,
    root_value_index: u8,
}

impl ColumnWindow {
    pub fn new(
        column_amount: u8,
        column_height: u8,
        window_height: u8,
        root_value_index: u8,
    ) -> Self {
        let columns = vec![vec![0; column_height]; column_amount];
        Self {
            columns,
            column_amount,
            column_height,
            window_height,
            root_value_index,
        }
    }
}

impl Default for ColumnWindow {
    fn default() -> Self {
        ColumnWindow::new(
            DefaultAmountOfColumns,
            DefaultColumnHeight,
            DefaultColumnHeight,
            DefaultRootValueIndex,
        )
    }
}
