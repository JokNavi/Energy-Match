use crate::data::details::indexes::{CorrectIndex, CorrectRanges};

mod data;
const SIDE_AMOUNT: i32 = 4;

fn main() {
    struct TestCorrectIndex;
    impl CorrectIndex for TestCorrectIndex{}
    println!("answer: {}", TestCorrectIndex::adjust_rotation(-4));
    println!("answer: {}", TestCorrectIndex::adjust_rotation(-3));
    println!("answer: {}", TestCorrectIndex::adjust_rotation(-2));
    println!("answer: {}", TestCorrectIndex::adjust_rotation(-1));
    println!("answer: {}", TestCorrectIndex::adjust_rotation(0));
    println!("answer: {}", TestCorrectIndex::adjust_rotation(1));
    println!("answer: {}", TestCorrectIndex::adjust_rotation(2));
    println!("answer: {}", TestCorrectIndex::adjust_rotation(3));
    println!("answer: {}", TestCorrectIndex::adjust_rotation(4));

    struct TestCorrectRanges;
    impl CorrectRanges for TestCorrectRanges{}

    println!("{:?}", TestCorrectRanges::get_range(-10, 10))
}
