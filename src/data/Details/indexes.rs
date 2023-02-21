pub trait CorrectIndex {
    fn correct_side_index(index: i32) -> i32 {
        match index {
            _ if index > crate::SIDE_AMOUNT => index % crate::SIDE_AMOUNT,
            _ if index < 0 => index.abs(),
            _ if index == 0 => crate::SIDE_AMOUNT,
            _ => index,
        }
    }

    fn check_row_index(index: i32) -> bool {
        if crate::ALLOW_0_INDEX == false {
            return index == 0;
        }
        return true;
    }
}
