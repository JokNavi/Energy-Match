use colored::{Color, ColoredString, Colorize};
use core::fmt;

pub struct ColoredText {
    pub color: colored::Color,
    pub text: String,
}

impl ColoredText {
    pub fn new(color: Color, text: String) -> Self {
        ColoredText { color, text }
    }
}

impl fmt::Display for ColoredText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colored_text = self.text.color(self.color);
        write!(f, "{}", colored_text)
    }
}

pub fn color_selector(code: i32) -> Result<Color, String> {
    match code {
        1 => Ok(Color::Red),
        2 => Ok(Color::Green),
        3 => Ok(Color::Blue),
        4 => Ok(Color::Yellow),
        _ => Err("Unknown color selector code".to_string()),
    }
}
