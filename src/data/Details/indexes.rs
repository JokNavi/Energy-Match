pub trait CorrectIndex {
    fn correct_side_index(index: i32) -> i32 {
        match index.abs() {
            _ if index > crate::SIDE_AMOUNT => index % crate::SIDE_AMOUNT,
            _ if index < 0 => index.abs(),
            _ if index == 0 => crate::SIDE_AMOUNT,
            _ => index,
        }
    }

    fn get_range(start: i32, end: i32) -> Vec<i32> {
        if start > end {
            (end..=start).collect()
        } else {
            (start..=end).collect()
        }
    }
}
