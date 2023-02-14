use super::shapes::Shape;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Game {
    pub local_rotations: i32,
    pub shape: Rc<Shape>,
    pub shape_collection: HashMap<i32, Rc<Shape>>,
}

impl Game {
    pub fn new() -> Self {
        let shape_collection = HashMap::from([(0, Rc::new(Shape::new(0)))]);
        Game {
            local_rotations: 0,
            shape: shape_collection[&0].clone(),
            shape_collection,
        }
    }

    fn get_side(&mut self, index: i32) -> Rc<Shape> {
        if let std::collections::hash_map::Entry::Vacant(e) = self.shape_collection.entry(index) {
            e.insert(Rc::new(Shape::new(index)));
            self.shape_collection[&index].clone()
        } else {
            self.shape_collection[&index].clone()
        }
    }

    pub fn swipe_left(&mut self, amount: i32) {
        self.shape = self.get_side(self.shape.index - amount);
        for i in 0..(self.shape.index + amount){
            match self.shape_collection.get(&i){
                None => continue,
                Some(shape) => self.local_rotations + shape.rotations,
            };
        }
    }

    pub fn swipe_right(&mut self, amount: i32) {
        self.shape = self.get_side(self.shape.index - amount);
        for i in 0..(self.shape.index - amount){
            match self.shape_collection.get(&i){
                None => continue,
                Some(shape) => self.local_rotations - shape.rotations,
            };
        }
    }
}
