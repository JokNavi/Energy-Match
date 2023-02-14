mod data;
use std::collections::HashMap;

use data::games::Game;

const SIDE_AMOUNT: i32 = 4;



fn main() {
    let mut game = Game::new();
    println!("{}", game.shape.rotations);
    game.shape.swipe_up(2);
    println!("{}", game.shape.rotations);
}
