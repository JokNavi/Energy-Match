use colored::{Color, ColoredString, Colorize};

use crate::data::details::indexes::CorrectIndex;

#[derive(Debug)]
pub struct RowSlice {
    pub rotations: Vec<i32>,
    index: i32,
    display_value: ColoredString,
}

impl CorrectIndex for RowSlice {}

impl RowSlice {
    pub fn new(rotations: i32, index: i32) -> Self {
        Self {
            rotations: vec![Self::adjust_rotation(rotations)],
            index,
            display_value: Self::create_side_color(index, rotations),
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

    pub fn add_rotation(&mut self, new_rotation: i32) {
        self.rotations.push(Self::adjust_rotation(new_rotation));
        self.display_value = Self::create_side_color(self.index, self.rotations());
    }

    pub fn rotations(&self) -> i32 {
        Self::adjust_rotation(self.rotations.iter().sum())
    }
}

impl PartialEq for RowSlice {
    fn eq(&self, other: &Self) -> bool {
        self.rotations() == other.rotations() && self.index == other.index
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
                rotations: vec![3],
                index: 0,
                display_value: RowSlice::create_side_color(3, 0),
            }
        );

        assert_eq!(
            RowSlice::new(-2, 0),
            RowSlice {
                rotations: vec![2],
                index: 0,
                display_value: RowSlice::create_side_color(2, 0),
            }
        );

        assert_eq!(
            RowSlice::new(5, 0),
            RowSlice {
                rotations: vec![1],
                index: 0,
                display_value: RowSlice::create_side_color(1, 0),
            }
        );

        assert_eq!(
            RowSlice::new(0, 0),
            RowSlice {
                rotations: vec![0],
                index: 0,
                display_value: RowSlice::create_side_color(0, 0),
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
        row_slice.add_rotation(2);
        assert_eq!(row_slice, RowSlice::new(3, 0));

        row_slice = RowSlice::new(1, 0);
        row_slice.add_rotation(5);
        assert_eq!(row_slice, RowSlice::new(2, 0));

        row_slice = RowSlice::new(0, 0);
        row_slice.add_rotation(-1);
        assert_eq!(row_slice, RowSlice::new(3, 0));
    }
}
