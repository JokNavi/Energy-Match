use game_logic::{
    games::{Game, LEVEL_SIZE, SIDE_AMOUNT},
    rows::Row,
};

pub mod game_logic;
pub mod traits;

fn main() {
    let row = Row::new(LEVEL_SIZE);
    match row.display_row(3) {
        Ok(_) => (),
        Err(err) => match err {
            traits::indexes::RowIndexError::AboveMax => {
                println!("Index is out of bounds (too high)")
            }
            traits::indexes::RowIndexError::UnderZero => println!("Index below 0"),
            traits::indexes::RowIndexError::NonI32Fitting => println!("Can't be converted to i32"),
        },
    };
}
