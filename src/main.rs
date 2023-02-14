mod data;
use std::collections::HashMap;

use data::games::Game;

const SIDE_AMOUNT: i32 = 4;



fn main() {
    let mut game = Game::new();
    println!("{}", game.local_rotations);
    game.swipe_left(5);
    println!("{}", game.local_rotations);
}
