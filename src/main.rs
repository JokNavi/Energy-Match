mod data;
mod display;
use data::bar;
const SIDE_AMOUNT: i32 = 4;

fn main() {
    let mut bar = data::bar::Bar::new();
    println!("Cube index: 0");
    bar.selected_cube.swipe_down(2);
    bar.swipe(bar::Direction::Left, 5);
    bar.swipe(bar::Direction::Right, 5);
    bar.selected_cube.swipe_up(2);
}
