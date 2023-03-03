use crate::traits::{
    indexes::{CorrectIndex, GenerateSlices, RowIndexError},
    patterns::ContainsPattern,
};

use super::games::DISPLAY_LENGTH;

pub struct Row {
    pub slices: Vec<i32>,
    pub index: i32,
    length: i32,
}

impl CorrectIndex for Row {}
impl GenerateSlices for Row {}

impl Row {
    pub fn new(length: i32) -> Self {
        Self {
            slices: Self::generate_slices(length),
            index: 0,
            length,
        }
    }

    pub fn get_slice<T>(&self, index: T) -> Option<i32>
    where
        T: TryInto<i32> + TryInto<usize> + Copy,
    {
        let index = match Self::validate_index(index) {
            Ok(value) => value,
            Err(_) => return None,
        };
        self.slices.get(index).copied()
    }

    pub fn set_slice(&mut self, value: i32) {
        self.slices[self.index as usize] = Self::adjust_rotation(value);
    }

    fn get_edge_line(&self, index: i32) -> String {
        let mut edge_line = "...=".to_string();
        for i in 0..DISPLAY_LENGTH {
            if Self::validate_index((index - 2) + i as i32).is_ok() {
                edge_line.push_str(&"=".repeat(7));
            };
        }
        edge_line.push_str("==...");
        edge_line
    }

    fn get_display_line(&self, index: i32) -> String {
        let mut display_line = "   |".to_string();
        for i in 0..DISPLAY_LENGTH {
            let usize_index = match Self::validate_index((index - 2) + i as i32) {
                Ok(value) => value,
                Err(_) => continue,
            };
            let value = self.get_slice(usize_index).unwrap();
            display_line.push_str(&format!(" [{value:^4}]"));
        }
        display_line.push_str(" |   ");
        display_line
    }

    pub fn display_row<T>(&self, index: T) -> Result<(), RowIndexError>
    where
        T: TryInto<i32> + TryInto<usize> + Copy,
    {
        let index: i32 = Self::validate_index(index)?.try_into().unwrap();
        let display_line = self.get_display_line(index);
        let edge_line = self.get_edge_line(index);
        println!("{}", edge_line);
        println!("{}", display_line);
        println!("{}", edge_line);
        Ok(())
    }
}

impl ContainsPattern for Row {
    fn contains_pattern(&self, pattern: Vec<i32>) -> bool {
        for window in self.slices.windows(pattern.len()) {
            if window == pattern.as_slice() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test_row {
    use core::panic;

    use crate::traits::indexes::RowIndexError;

    use super::Row;

    #[test]
    fn get_slice() {
        let row = Row::new(5);
        assert_eq!(row.get_slice(0).unwrap(), row.slices[0]);
        assert_eq!(row.get_slice(10), None);
    }

    #[test]
    fn set_slice() {
        let mut row = Row::new(50);
        row.set_slice(5);
        assert_eq!(row.get_slice(0), Some(1));
        row.set_slice(3);
        assert_eq!(row.get_slice(0), Some(3));
    }

    #[test]
    fn get_edge_line() {
        let row = Row::new(50);
        assert_eq!(row.get_edge_line(-1), "...=================...");
        assert_eq!(row.get_edge_line(row.length), "...=================...");

        assert_eq!(
            row.get_edge_line(3),
            "...======================================..."
        );
        assert_eq!(
            row.get_edge_line(1),
            "...===============================..."
        );
        assert_eq!(row.get_edge_line(0), "...========================...");

        assert_eq!(
            row.get_edge_line(row.length - 3),
            "...======================================..."
        );
        assert_eq!(
            row.get_edge_line(row.length - 2),
            "...===============================..."
        );
        assert_eq!(
            row.get_edge_line(row.length - 1),
            "...========================..."
        );
    }

    #[test]
    fn display_row() {
        let row = Row::new(50);
        assert_eq!(row.display_row(-1), Err(RowIndexError::UnderZero));
        assert_eq!(row.display_row(0), Ok(()));
        assert_eq!(row.display_row(row.length), Err(RowIndexError::AboveMax));
    }
}
