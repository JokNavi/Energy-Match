use super::shapes::Shape;
use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Clone)]
pub struct Game {
    pub local_rotations: i32,
    pub shape: Shape,
    pub shape_collection: HashMap<i32, Shape>,
}

impl Game {
    pub fn new() -> Self {
        let shape_collection = HashMap::from([(0, Shape::new(0))]);
        Game {
            local_rotations: 0,
            shape: shape_collection[&0].clone(),
            shape_collection,
        }
    }

    fn get_shape(&mut self, index: i32) -> Shape {
        if let std::collections::hash_map::Entry::Vacant(e) = self.shape_collection.entry(index) {
            e.insert(Shape::new(index));
            self.shape_collection[&index].clone()
        } else {
            self.shape_collection[&index].clone()
        }
    }

    fn get_range(start: i32, end: i32) -> Vec<i32> {
        if start > end {
            (end..=start).collect()
        } else {
            (start..=end).collect()
        }
    }

    pub fn swipe_left(&mut self, amount: i32) {
        for i in Self::get_range(self.shape.index, self.shape.index + amount) {
            //println!("i: {}", self.shape.index - i);
            match self.shape_collection.get(&i) {
                None => continue,
                Some(shape) => self.local_rotations += shape.rotations,
            }
        }
        self.shape = self.get_shape(self.shape.index + amount);
    }

    pub fn swipe_right(&mut self, amount: i32) {
        for i in Self::get_range(self.shape.index, self.shape.index - amount) {
            //println!("i: {i}");
            match self.shape_collection.get(&i) {
                None => continue,
                Some(shape) => self.local_rotations -= shape.rotations,
            }
        }
        self.shape = self.get_shape(self.shape.index - amount);
    }

    pub fn check_pattern_exists(&mut self, pattern: Vec<i32>) -> bool {
        self.shape_collection
            .values()
            .map(|value| &value.rotations)
            .zip(&pattern)
            .filter(|&(a, b)| a == b)
            .count()
            == pattern.len()
    }

    fn ask_input(input_text: String, error_text: String) -> Result<String, std::io::Error> {
        print!("{input_text}");
        if let Err(err) = io::stdout().flush() {
            println!("{error_text}, {err}");
            return Ok(Self::ask_input(input_text, error_text).expect("Function can only return OK(). "));
        };
        let mut response = String::new();
        if let Err(err) = io::stdin().read_line(&mut response) {
            println!("{error_text}, {err}");
            return Ok(Self::ask_input(input_text, error_text).expect("Function can only return OK(). "));
        };
        Ok(response)
    }

    pub fn print_game_snippet(&mut self){
        let left_cube = self.get_shape(self.shape.index-1);
        let middle_cube = self.shape.clone();
        let right_cube = self.get_shape(self.shape.index+1);

        println!("       ____ ____ _____    ");
        println!("      / {0:^2} / {1:^2} / {2:^2} /|     ", Shape::adjust_index(left_cube.rotations+1), Shape::adjust_index(middle_cube.rotations+1), Shape::adjust_index(right_cube.rotations+1),);
        println!("/⎺⎺⎺⎺ | {0:^2} | {1:^2} | {2:^2} |/⎺⎺⎺⎺/", left_cube.rotations, middle_cube.rotations, right_cube.rotations,);
        println!("⎺⎺⎺⎺⎺ ⎺⎺⎺⎺⎺ ⎺⎺⎺⎺ ⎺⎺⎺⎺ ⎺⎺⎺⎺⎺");
        
    }

    pub fn game_loop() -> Result<(), String> {
        println!("Starting game...");
        Ok(())
    }
}
