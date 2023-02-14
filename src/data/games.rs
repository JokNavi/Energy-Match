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
        if self.shape_collection.contains_key(&index) {
            self.shape_collection[&index].clone()
        } else {
            self.shape_collection
                .insert(index, Rc::new(Shape::new(index)));
            self.shape_collection[&index].clone()
        }
    }
}
