use crate::data::{details::indexes::CorrectIndex, visual_objects::row_slices::RowSlice};

mod data;
const SIDE_AMOUNT: i32 = 4;

fn main() {
    struct TestCorrectIndex;
    impl CorrectIndex for TestCorrectIndex{}

    let mut row_slice = RowSlice::new(0, 0);
    println!("Side rotations: {}.", row_slice.rotations);
    row_slice.add_rotation(1);
    println!("Side rotations: {}.", row_slice.rotations);
    row_slice.add_rotation(1);
    println!("Side rotations: {}.", row_slice.rotations);
    row_slice.remove_rotation(2);
    println!("Side rotations: {}.", row_slice.rotations);
}
