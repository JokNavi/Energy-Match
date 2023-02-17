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
    let mut pattern: Vec<i32> = Vec::<i32>::new();
    let mut game = Game::new();
    debug(&game, "Init");
    game.swipe_left(1);
    pattern.push(game.shape.rotations);
    debug(&game, "game.swipe_right(1);");
    println!("{}",game.check_pattern_exists(pattern));
}
