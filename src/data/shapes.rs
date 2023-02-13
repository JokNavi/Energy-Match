use super::sides::Side;
use std::collections::HashMap;

pub struct Shape {
    pub index: i32,
    pub side: Side,
    pub side_collection: HashMap<i32, Side>,
}

impl Shape {
    pub fn new(index: i32) -> Self {
        Shape {
            index,
            side: Side::new(index),
            side_collection: HashMap::from([(index, Side::new(index))])
        }
    }

    pub fn get_side(&mut self, index: i32) -> Side {
        if self.side_collection.contains_key(&index) {
            self.side_collection[&index]
        }else {
            self.side_collection.insert(index, Side::new(index));
            Side::new(index)
        }
    }
}
