mod data;
use data::{games::Game, shapes::Shape};

const SIDE_AMOUNT: i32 = 4;

fn main() {
    let mut game = Game::new();
    game.get_shape(1);
    game.get_shape(2);
}
