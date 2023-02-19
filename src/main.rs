mod data;
use data::{games::Game};

const SIDE_AMOUNT: i32 = 4;

fn main() {
    let mut game = Game::new();
    game.game_loop()
    
}
