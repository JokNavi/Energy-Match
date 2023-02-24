pub trait CorrectIndex {
    fn adjust_rotation(rotation: i32) -> i32 {
        let rotation = rotation.abs();
        if rotation > 0 && rotation < crate::SIDE_AMOUNT{
            return rotation;
        } rotation % crate::SIDE_AMOUNT
    }
}

pub trait CorrectRanges {
    fn get_range(start: i32, end: i32) -> Vec<i32> {
        if start > end {
            (end..=start).collect()
        } else {
            (start..=end).collect()
        }
    }
}

#[cfg(test)]
pub mod correct_index_tests {
    use super::{CorrectIndex, CorrectRanges};
    struct TestCorrectIndex;
    impl CorrectIndex for TestCorrectIndex {}
    struct TestCorrectRanges;
    impl CorrectRanges for TestCorrectRanges {}

    #[test]
    fn correct_side_index() {
        assert_eq!(TestCorrectIndex::adjust_rotation(2), 2);
        assert_eq!(TestCorrectIndex::adjust_rotation(0), 0);
        assert_eq!(TestCorrectIndex::adjust_rotation(-3), 3);
        assert_eq!(TestCorrectIndex::adjust_rotation(-8), 0);
        assert_eq!(TestCorrectIndex::adjust_rotation(-8), 0);
    }

    #[test]
    fn get_range() {
        assert_eq!(TestCorrectRanges::get_range(0, 5), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(TestCorrectRanges::get_range(5, 0), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(TestCorrectRanges::get_range(5, 5), vec![5]);
        assert_eq!(TestCorrectRanges::get_range(-5, -3), vec![-5, -4, -3]);
    }
}
