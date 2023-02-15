use super::shapes::Shape;
use std::collections::HashMap;

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

    fn get_side(&mut self, index: i32) -> Shape {
        if let std::collections::hash_map::Entry::Vacant(e) = self.shape_collection.entry(index) {
            e.insert(Shape::new(index));
            self.shape_collection[&index].clone()
        } else {
            self.shape_collection[&index].clone()
        }
    }

    pub fn swipe_left(&mut self, amount: i32) {
        for i in self.shape.index..(self.shape.index + amount){
            match self.shape_collection.get(&i){
                None => continue,
                Some(shape) => self.local_rotations += self.shape.rotations,
            };
        }
        self.shape = self.get_side(self.shape.index + amount);
    }

    pub fn swipe_right(&mut self, amount: i32) {
        
        for i in self.shape.index..(self.shape.index - amount){
            match self.shape_collection.get(&i){
                None => continue,
                Some(shape) => self.local_rotations -= self.shape.rotations,
            };
        }
        self.shape = self.get_side(self.shape.index - amount);
    }
}
