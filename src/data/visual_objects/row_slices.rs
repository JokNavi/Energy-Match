use colored::{Color, ColoredString, Colorize};

use crate::data::details::indexes::CorrectIndex;

#[derive(Debug)]
pub struct RowSlice {
    rotations: i32,
    index: i32,
    display_value: ColoredString,
}

impl CorrectIndex for RowSlice {}

impl RowSlice {
    pub fn new(rotation: i32, index: i32) -> Self {
        Self {
            rotations: rotation,
            index,
            display_value: Self::create_side_color(index, rotation),
        }
    }

    fn create_side_color(rotations: i32, index: i32) -> ColoredString {
        match rotations {
            0 => index.to_string().color(Color::Red),
            1 => index.to_string().color(Color::Green),
            2 => index.to_string().color(Color::Blue),
            3 => index.to_string().color(Color::Yellow),
            _ => index.to_string().normal(),
        }
    }

    pub fn rotations(&self) -> i32 {
        Self::adjust_rotation(self.rotations)
    }

    pub fn append_rotation(&mut self, amount: i32) {
        self.rotations += amount;
        self.display_value =
            Self::create_side_color(self.index, Self::adjust_rotation(self.rotations));
    }
}

impl PartialEq for RowSlice {
    fn eq(&self, other: &Self) -> bool {
        self.rotations == other.rotations && self.index == other.index
    }
}

#[cfg(test)]
mod row_slice_tests {
    use super::RowSlice;
    use colored::Colorize;

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
    }

    #[test]
    fn create_side_color() {
        assert_eq!(RowSlice::create_side_color(0, 0), 0.to_string().red());
        assert_eq!(RowSlice::create_side_color(3, 0), 0.to_string().yellow());
        assert_eq!(RowSlice::create_side_color(-1, 0), 0.to_string().normal());
        assert_eq!(RowSlice::create_side_color(4, 0), 0.to_string().normal());
    }

    #[test]
    fn append_rotation() {
        let mut row_slice = RowSlice::new(0, 0);
        row_slice.append_rotation(2);
        assert_eq!(row_slice.rotations(), 2);

        row_slice = RowSlice::new(1, 0);
        row_slice.append_rotation(3);
        assert_eq!(row_slice.rotations(), 0);

        row_slice = RowSlice::new(0, 0);
        row_slice.append_rotation(5);
        assert_eq!(row_slice.rotations(), 1);
    }

    #[test]
    fn rotations() {
        let mut row_slice = RowSlice::new(0, 0);
        assert_eq!(row_slice.rotations(), 0);

        row_slice = RowSlice::new(10, 0);
        assert_eq!(row_slice.rotations(), 2);

        row_slice = RowSlice::new(0, 0);
        assert_eq!(row_slice.rotations(), 0);
    }
}