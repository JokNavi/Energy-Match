use colored::{ColoredString, Colorize};

#[derive(Clone)]
pub struct Side {
    pub side: i32,
    pub colour: ColoredString,
}

impl Side {
    pub fn new(side: i32) -> Self {
        Side {
            side,
            colour: side.to_string().color(Self::colours(side)),
        }
    }

    fn colours(accesor: i32) -> String{
        println!("{}", &accesor);
        match accesor{
            1 => "red".to_string(),
            2 => "blue".to_string(),
            3 => "green".to_string(),
            4 => "yellow".to_string(),
            _ => panic!("Invalid Colour function."),
        }
    }
}
