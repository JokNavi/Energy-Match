use super::sides::Side;
use rand::Rng;
use std::collections::HashMap;

pub struct Shape {
    pub index: i32,
    pub rotations: i32,
    pub side_collection: HashMap<i32, Side>,
}

impl Shape {
    pub fn new(index: i32) -> Self {
        let rotations = rand::thread_rng().gen_range(1..=crate::SIDE_AMOUNT);
        let mut side_collection: HashMap<i32, Side> = HashMap::new();
        side_collection.insert(index, Side::new(index));
        Shape {
            index,
            rotations,
            side_collection,
        }
    }

    pub fn get_side(&mut self) -> &mut Side {
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.side_collection.entry(self.rotations)
        {
            e.insert(Side::new(self.rotations));
            self.side_collection.get_mut(&self.rotations).unwrap()
        } else {
            self.side_collection.get_mut(&self.rotations).unwrap()
        }
    }

    pub fn swipe_up(&mut self, amount: i32) {
        self.rotations = Self::adjust_index(self.rotations - amount);
    }

    pub fn swipe_down(&mut self, amount: i32) {
        self.rotations = Self::adjust_index(self.rotations - amount);
    }

    pub fn adjust_index(index: i32) -> i32 {
        match index {
            _ if index > crate::SIDE_AMOUNT => index % crate::SIDE_AMOUNT,
            _ if index < 0 => index.abs(),
            _ if index == 0 => crate::SIDE_AMOUNT,
            _ => index,
        }
    }
}
