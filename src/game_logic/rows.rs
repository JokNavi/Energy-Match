use crate::traits::indexes::{CorrectIndex, GenerateSlices};

pub struct Row {
    pub slices: Vec<i32>,
    index: i32,
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

    pub fn get_slice(&self, index: i32) -> Option<i32> {
        self.slices.get(index as usize).copied()
    }

    pub fn set_slice<T>(&mut self, index: T, value: i32) -> Result<(), String>
    where
        T: TryInto<i32> + TryInto<usize> + Copy,
    {
        let index = match Self::validate_index(index) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };
        let slice = self.slices.get_mut(index).unwrap();
        *slice = Self::adjust_rotation(value);
        Ok(())
    }
}

#[cfg(test)]
mod TestRow {
    use super::Row;

    #[test]
    fn get_slice() {
        let row = Row::new(5);
        assert_eq!(row.get_slice(0).unwrap(), row.slices[0]);
        assert_eq!(row.get_slice(10), None);
    }

    #[test]
    fn set_slice() {
        let mut row = Row::new(5);
        row.set_slice(0, 1);
        assert_eq!(row.get_slice(0).unwrap(), 1);

        assert_eq!(
            row.set_slice(10, 5),
            Err("Index is out of bounds".to_string())
        );
    }
}
