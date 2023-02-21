use colored::{Color, Colorize};
use core::fmt;

pub struct ColoredText {
    color: colored::Color,
    text: String,
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

pub fn color_selector(code: i32) -> Color {
    match code {
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Blue,
        4 => Color::Yellow,
        _ => Color::White,
    }
}
