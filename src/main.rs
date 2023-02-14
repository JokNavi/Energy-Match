mod data;
use data::shapes::Shape;

const SIDE_AMOUNT: i32 = 4;

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {}
