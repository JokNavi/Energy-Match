use data::{visual_objects::rows::Row, details::displays::RowDisplay};

mod data;

const SIDE_AMOUNT: i32 = 4;

fn main() {
    let mut row = Row::new();
    println!("{}", RowDisplay::new(&mut row, 0, 2));
}
