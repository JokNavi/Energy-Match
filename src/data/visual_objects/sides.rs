use crate::data::Details::colors::ColoredText;
use colored::Color;

pub struct Side {
    pub index: i32,
    pub display_value: ColoredText,
}

impl Side {
    pub fn new(index: i32, color: Color) -> Self {
        Side {
            index,
            display_value: ColoredText::new(color, index.to_string()),
        }
    }
}
