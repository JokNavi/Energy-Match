use colored::{ColoredString, Colorize};

#[derive(Clone)]
pub struct Side {
    pub index: i32,
    pub colour: ColoredString,
}

impl Side {
    pub fn new(index: i32, rotations: i32) -> Self {
        Side {
            index,
            colour: index.to_string().color(colours(rotations)),
        }
    }

}


