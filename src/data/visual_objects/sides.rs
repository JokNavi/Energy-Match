use crate::data::colors::{ColoredText, color_selector};

pub struct Side {
    pub index: i32,
    pub display_value: ColoredText,
}

impl Side {
    pub fn new(index: i32, rotations: i32) -> Self {
        Side {
            index,
            display_value: ColoredText::new(color_selector(rotations), index.to_string()),
        }
    }

}