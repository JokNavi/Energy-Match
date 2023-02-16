use super::shapes::Shape;
use std::collections::{HashMap, HashSet};

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
            return (end..=start).collect();
        } else {
            return (start..=end).collect();
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

    pub fn contains_index(&mut self, index: i32) -> bool {
        matches!(self.shape_collection.get_key_value(&index), Some(_))
    }

    pub fn check_pattern_exists(&mut self, pattern: Vec<i32>) -> bool {
        self.shape_collection.keys().zip(&pattern).filter(|&(a, b)| a == b).count() == pattern.len()
    }
}
