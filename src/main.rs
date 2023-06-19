use game_logic::games::Game;

pub mod game_logic;
pub mod traits;

fn main() {
    let mut game = Game::new();
    game.start()
}
