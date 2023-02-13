use super::sides::Side;
use std::collections::HashMap;

pub struct Shape {
    pub index: i32,
    pub rotations: i32,
    pub side: Side,
    pub side_collection: HashMap<i32, Side>,
}

impl Shape {
    pub fn new(index: i32) -> Self {
        Shape {
            index,
            rotations: 0,
            side: Side::new(index),
            side_collection: HashMap::from([(index, Side::new(index))]),
        }
    }

    fn get_side(&mut self, index: i32) -> Side {
        if let std::collections::hash_map::Entry::Vacant(e) = self.side_collection.entry(index) {
            e.insert(Side::new(index));
            Side::new(index)
        } else {
            self.side_collection[&index]
        }
    }

    pub fn swipe_up(&mut self, amount: i32) {
        let adjusted_index = Self::adjust_index(self.side.index + amount);
        self.side = self.get_side(adjusted_index);
    }

    pub fn swipe_down(&mut self, amount: i32) {
        let adjusted_index = Self::adjust_index(self.side.index - amount);
        self.side = self.get_side(adjusted_index);
    }

    fn adjust_index(index: i32) -> i32 {
        match index {
            _ if index > crate::SIDE_AMOUNT => index % crate::SIDE_AMOUNT,
            _ if index <= 0 => index.abs(),
            _ => index,
        }
    }
}
