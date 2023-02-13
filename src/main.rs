mod data;
use data::shapes::Shape;

const SIDE_AMOUNT: i32 = 4;

fn main() {
    println!("Starting program...");
    let mut shape = Shape::new(1);
    println!("{}", shape.side.index);
    shape.swipe_up(3);
    println!("{}", shape.side.index);
    shape.swipe_down(2);
    println!("{}", shape.side.index);
    
}
