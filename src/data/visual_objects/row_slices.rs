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
        let rotations = Self::correct_side_index(rotations);
        Self {
            rotations,
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

    pub fn set_rotation(&mut self, rotations: i32) {
        let rotations = Self::correct_side_index(rotations);
        self.rotations = rotations;
        self.display_value = Self::create_side_color(self.index, self.rotations);
    }
}

impl PartialEq for RowSlice {
    fn eq(&self, other: &Self) -> bool {
        self.rotations == other.rotations && self.index == other.index
    }
}

#[cfg(test)]
pub mod row_slice_tests {
    use colored::Colorize;

    use super::RowSlice;

    #[test]
    fn new() {
        assert_eq!(
            RowSlice::new(3, 0),
            RowSlice {
                rotations: 3,
                index: 0,
                display_value: RowSlice::create_side_color(3, 0),
            }
        );

        assert_eq!(
            RowSlice::new(-2, 0),
            RowSlice {
                rotations: 2,
                index: 0,
                display_value: RowSlice::create_side_color(2, 0),
            }
        );

        assert_eq!(
            RowSlice::new(5, 0),
            RowSlice {
                rotations: 1,
                index: 0,
                display_value: RowSlice::create_side_color(1, 0),
            }
        );

        assert_eq!(
            RowSlice::new(0, 0),
            RowSlice {
                rotations: 1,
                index: 0,
                display_value: RowSlice::create_side_color(1, 0),
            }
        );
    }

    #[test]
    fn create_side_color() {
        assert_eq!(RowSlice::create_side_color(3, 0), 0.to_string().blue());
        assert_eq!(RowSlice::create_side_color(5, 0), 0.to_string().normal());
        assert_eq!(RowSlice::create_side_color(0, 0), 0.to_string().normal());
    }

    #[test]
    fn set_rotation() {
        let mut row_slice = RowSlice::new(1, 0);
        row_slice.set_rotation(2);
        assert_eq!(row_slice, RowSlice::new(2, 0));

        row_slice = RowSlice::new(1, 0);
        row_slice.set_rotation(5);
        assert_eq!(row_slice, RowSlice::new(1, 0));
    }
}
