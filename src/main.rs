mod data;
use data::{games::Game, shapes::Shape};
use std::collections::HashMap;

const SIDE_AMOUNT: i32 = 4;

fn main() {
    let mut shape = Shape::new(0);
    println!("{}", shape.rotations);
    shape.swipe_up(1);
    println!("{}", shape.get_side().index);
}
