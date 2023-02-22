use core::fmt;
use crate::data::details::colors::ColoredText;
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

    pub fn display(&self) -> colored::ColoredString {
        self.display_value.colored_text()
    }
    
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_value)
    }
}

impl fmt::Debug for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(I: {}, V: {})", self.index, self.display_value)
    }
}
