use crate::data::{details::indexes::CorrectIndex, visual_objects::row_slices::RowSlice};

mod data;
const SIDE_AMOUNT: i32 = 4;

fn main() {
    /*
    let mut row_slice = RowSlice::new(1, 0);
    println!(
        "{}, rotations: {:?}",
        row_slice.rotations(),
        row_slice.rotations
    );
    for _ in 1..9 {
        row_slice.add_rotation(1);
        println!(
            "{}, rotations: {:?}",
            row_slice.rotations(),
            row_slice.rotations
        );
    }
    */

    struct TestStruct();
    impl CorrectIndex for TestStruct {}
    println!("{}", TestStruct::adjust_rotation(4));
}
