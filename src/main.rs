use game_logic::games::{SIDE_AMOUNT, Game};

pub mod game_logic;
pub mod traits;

fn main() {
    let mut game = Game::new();
    game.display_row(1);
}
