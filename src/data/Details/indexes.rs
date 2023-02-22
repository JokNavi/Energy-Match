pub trait CorrectIndex {
    fn correct_side_index(index: i32) -> i32 {
        match index {
            _ if index > crate::SIDE_AMOUNT => index % crate::SIDE_AMOUNT,
            _ if index < 0 => index.abs(),
            _ if index == 0 => crate::SIDE_AMOUNT,
            _ => index,
        }
    }

}
