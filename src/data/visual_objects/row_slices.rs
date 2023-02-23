use colored::{Color, ColoredString, Colorize};
use rand::Rng;

use crate::data::details::indexes::CorrectIndex;

#[derive(Debug)]
pub struct RowSlice {
    rotations: i32,
    index: i32,
    display_value: ColoredString,
}

impl CorrectIndex for RowSlice {}

impl RowSlice {
    pub fn new(rotations: i32, index: i32) -> Self {
        Self {
            rotations: Self::correct_side_index(rotations),
            index,
            display_value: Self::create_side_color(index, rotations),
        }
    }

    pub fn new_rand(index: i32) -> Self {
        let rotations = rand::thread_rng().gen_range(1..=crate::SIDE_AMOUNT);
        Self::new(rotations, index)
    }

    fn create_side_color(rotations: i32, index: i32) -> ColoredString {
        match rotations {
            1 => index.to_string().color(Color::Red),
            2 => index.to_string().color(Color::Green),
            3 => index.to_string().color(Color::Blue),
            4 => index.to_string().color(Color::Yellow),
            _ => index.to_string().normal(),
        }
    }
}

impl PartialEq for RowSlice {
    fn eq(&self, other: &Self) -> bool {
        self.rotations == other.rotations && self.index == other.index
    }
}
