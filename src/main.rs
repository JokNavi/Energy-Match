mod data;
use data::games::Game;

const SIDE_AMOUNT: i32 = 4;

fn debug(game: &Game, action: &str) {
    println!("\nDid {action}");
    println!("game.shape.index: {}", game.shape.index);
    println!("game.shape.rotations: {}", game.shape.rotations);
    println!("game.local_rotations: {}", game.local_rotations);
}

fn main() {
    let mut game = Game::new();
    game.swipe_left(1);
    game.swipe_right(1);
    game.print_game_snippet();
}
