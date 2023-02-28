use game_logic::games::{Game, LEVEL_SIZE, SIDE_AMOUNT};

pub mod game_logic;
pub mod traits;

fn main() {
    let game = Game::new();
    game.display_row(-3);

}


