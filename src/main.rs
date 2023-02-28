use game_logic::games::{SIDE_AMOUNT, Game, LEVEL_SIZE};

pub mod game_logic;
pub mod traits;

fn main() {
    let mut game = Game::new();
    game.display_row(LEVEL_SIZE-1);
}
