use super::sides::Side;
use rand::Rng;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Shape {
    pub index: i32,
    pub rotations: i32,
    pub side: Rc<Side>,
    pub side_collection: HashMap<i32, Rc<Side>>,
}

impl Shape {
    pub fn new(index: i32) -> Self {
        let rotations = rand::thread_rng().gen_range(1..crate::SIDE_AMOUNT);
        let side_collection = HashMap::from([(rotations, Rc::new(Side::new(rotations)))]);
        Shape {
            index,
            rotations,
            side: side_collection[&rotations].clone(),
            side_collection,
        }
    }

    fn get_side(&mut self, index: i32) -> Rc<Side> {
        if let std::collections::hash_map::Entry::Vacant(e) = self.side_collection.entry(index) {
            e.insert(Rc::new(Side::new(index)));
            self.side_collection[&index].clone()
        } else {
            self.side_collection[&index].clone()
        }
    }

    pub fn swipe_up(&mut self, amount: i32) {
        self.rotations = Self::adjust_index(self.rotations - amount); 
        self.side = self.get_side(self.rotations)
    }

    pub fn swipe_down(&mut self, amount: i32) {
        self.rotations = Self::adjust_index(self.rotations + amount); 
        self.side = self.get_side(self.rotations)
    }

    fn adjust_index(index: i32) -> i32 {
        match index {
            _ if index > crate::SIDE_AMOUNT => index % crate::SIDE_AMOUNT,
            _ if index <= 0 => index.abs(),
            _ => index,
        }
    }
}
