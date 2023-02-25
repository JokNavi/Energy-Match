use crate::data::details::indexes::CorrectIndex;

pub struct Row{
    slices: Vec<i32>,
    index: i32,
}

impl CorrectIndex for Row{}

impl Row{
    pub fn new() -> Self {
        Self {
            slices: Vec::new(),
            index: 0,
        }
    }
    
    fn generate_slice() -> i32{
        (0..length)
            .map(|_| rand::thread_rng().gen_range(0..=SIDE_AMOUNT - 1))
            .collect()
    }
}