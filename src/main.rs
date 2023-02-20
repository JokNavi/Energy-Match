mod data;
use std::collections::HashMap;

use data::games::Game;

const SIDE_AMOUNT: i32 = 4;
const COLOURS: HashMap<i32, String> = HashMap::from([
    (1, "red".to_string()),
    (2, "blue".to_string()),
    (1, "green".to_string()),
    (1, "yellow".to_string()),
]);

fn main() {
    let mut game = Game::new();
    game.game_loop()
}
