use core::fmt;

use crate::data::visual_objects::rows::Row;

use super::{colors::ColoredText, indexes::CorrectIndex};

/*
        println!("       ____ ____ _____      ");
        println!("      /____/____/____/|     ",);
        println!("/⎺⎺⎺⎺ | {left_cube:^2} | {middle_cube:^2} | {right_cube:^2} |/⎺⎺⎺⎺/",);
        println!("⎺⎺⎺⎺⎺ ⎺⎺⎺⎺⎺ ⎺⎺⎺⎺ ⎺⎺⎺⎺ ⎺⎺⎺⎺⎺");
*/

type RowSliceDisplay = (Vec<String>, Vec<String>, Vec<String>, Vec<String>);

pub struct RowDisplay {
    center_index: i32,
    width: i32,
    layers: RowSliceDisplay,
}

impl RowDisplay {
    pub fn new(mut row: Row, center_index: i32, width: i32) -> Self {
        let mut layers = Self::push_prefix((Vec::new(), Vec::new(), Vec::new(), Vec::new()));
        for i in Row::get_range(center_index - width, center_index + width + 1) {
            layers = Self::push_shape(
                layers,
                &row.get_row_slice(i).get_current_side().display_value,
            );
        }
        layers = Self::push_postfix(layers);
        Self {
            center_index,
            width,
            layers,
        }
    }

    fn push_prefix(mut row_slice_display: RowSliceDisplay) -> RowSliceDisplay {
        row_slice_display.0.push("      ".to_string());
        row_slice_display.1.push("      ".to_string());
        row_slice_display.2.push("/⎺⎺⎺⎺ ".to_string());
        row_slice_display.3.push("⎺⎺⎺⎺⎺ ".to_string());
        row_slice_display
    }

    fn push_postfix(mut row_slice_display: RowSliceDisplay) -> RowSliceDisplay {
        row_slice_display.0.push("       ".to_string());
        row_slice_display.1.push("|      ".to_string());
        row_slice_display.2.push("|/⎺⎺⎺⎺/".to_string());
        row_slice_display.3.push("⎺⎺⎺⎺⎺  ".to_string());
        row_slice_display
    }

    fn push_shape(mut row_slice_display: RowSliceDisplay, side: &ColoredText) -> RowSliceDisplay {
        row_slice_display.0.push(" ____".to_string());
        row_slice_display.1.push("/____".to_string());
        row_slice_display.2.push(format!("| {side:^2} "));
        row_slice_display.3.push("⎺⎺⎺⎺⎺ ".to_string());
        row_slice_display
    }
}

impl fmt::Display for RowDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}", self.layers.0.join(""))?;
        writeln!(f, "{}", self.layers.1.join(""))?;
        writeln!(f, "{}", self.layers.2.join(""))?;
        writeln!(f, "{}", self.layers.3.join(""))
    }
}
