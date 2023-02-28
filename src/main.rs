use game_logic::{games::{Game, LEVEL_SIZE, SIDE_AMOUNT}, rows::Row};

pub mod game_logic;
pub mod traits;

fn main() {
    let row = Row::new(LEVEL_SIZE);
    match row.display_row(3){
        Ok(_) => (),
        Err(err) => println!("{}", err),
    };

}


