mod data;
mod display;
use data::bar;
const SIDE_AMOUNT: i32 = 7;

fn main() {
    let mut bar = data::bar::Bar::new();
    bar.swipe(bar::Direction::Left, 5);
    //display::print_cube::print_cube(&bar.selected_cube, 5);
    bar.swipe(bar::Direction::Right, 10);
    bar.swipe(bar::Direction::Left, 10);

}
