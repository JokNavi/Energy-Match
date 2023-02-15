mod data;
use data::games::Game;

const SIDE_AMOUNT: i32 = 4;


fn debug(game: &Game, action: &str){
    println!("game.shape.index: {}", game.shape.index);
    println!("game.shape.rotations: {}", game.shape.rotations);
    println!("game.local_rotations: {}", game.local_rotations);
    println!("DID {}", action);
}

fn main() {
    let mut game = Game::new();
    debug(&game, "Init");
    game.swipe_left(1);
    debug(&game, "game.swipe_left(1);");
}
