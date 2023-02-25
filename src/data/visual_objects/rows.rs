use crate::data::details::indexes::GenerateSlices;


pub struct Row {
    slices: Vec<i32>,
    index: i32,
}

impl GenerateSlices for Row {}

impl Row {
    pub fn new(length: i32) -> Self {
        Self {
            slices: Self::generate_slices(length),
            index: 0,
        }
    }

    pub fn get_slice(&self, index: usize) -> Option<i32> {
        self.slices.get(index).copied()
    }

    pub fn set_slice(&mut self, index: usize, value: i32) -> Result<i32, String> {

    }
}

#[cfg(test)]
mod TestRow{
    use super::Row;

    #[test]
    fn get_slice(){
        let row = Row::new(50);
        assert_eq!(row.get_slice(0).unwrap(), row.slices[0]);
        assert_eq!(row.get_slice(100), None);
    }
}
