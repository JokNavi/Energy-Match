use core::fmt;
use colored::{Colorize, Color};

struct ColoredText {
    color: colored::Color,
    text: String,
}

impl fmt::Display for ColoredText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colored_text = self.text.color(self.color);
        write!(f, "{}", colored_text)
    }
}

impl ColoredText {
    pub fn new(color: Color, text: String) -> Self{
        ColoredText { color, text, }
    }

    pub fn color(&mut self, new_color: Color){
        self.color = new_color;
    }

    pub fn color(&mut self, new_color: i32){
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
    }

}