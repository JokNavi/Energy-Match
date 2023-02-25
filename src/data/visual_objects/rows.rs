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
    
}