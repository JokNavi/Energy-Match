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

pub fn colours(accesor: i32) -> String{
    println!("{}", &accesor);
    match accesor{
        1 => "red".to_string(),
        2 => "blue".to_string(),
        3 => "green".to_string(),
        4 => "yellow".to_string(),
        _ => panic!("Invalid Colour function."),
    }
}
