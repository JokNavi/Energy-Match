pub mod test;
pub mod window;

fn main() {
    dbg!((3_f32 / 2_f32) as i8);
    let mut game = test::Game {
        current_game_values: vec![1, 2, 3],
    };

    // Call view() method with an immutable reference to game
    let view_result = game.view(1);
    println!("{:?}", view_result);

    // The immutable reference goes out of scope after the above block, so you can create a mutable reference to game
    game.swipe_up(1);

    // Call view() again to see the updated values
    let view_result_after_swipe = game.view(1);
    println!("{:?}", view_result_after_swipe);
}
