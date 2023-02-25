use rand::Rng;

use crate::data::game_logic::games::SIDE_AMOUNT;

pub trait CorrectIndex {
    fn adjust_rotation(rotation: i32) -> i32 {
        if rotation >= 0 {
            return rotation.abs() % SIDE_AMOUNT;
        }
        (SIDE_AMOUNT - (rotation.abs() % SIDE_AMOUNT)) % SIDE_AMOUNT
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


pub trait GenerateSlices {
    fn generate_slices(length: i32) -> Vec<i32>{
        (0..length)
            .map(|_| rand::thread_rng().gen_range(0..=SIDE_AMOUNT - 1))
            .collect()
    }
}

#[cfg(test)]
mod correct_index_tests {
    use super::{CorrectIndex, CorrectRanges};
    struct TestCorrectIndex;
    impl CorrectIndex for TestCorrectIndex {}
    struct TestCorrectRanges;
    impl CorrectRanges for TestCorrectRanges {}

    #[test]
    fn correct_side_index() {
        assert_eq!(TestCorrectIndex::adjust_rotation(2), 2);
        assert_eq!(TestCorrectIndex::adjust_rotation(0), 0);
        assert_eq!(TestCorrectIndex::adjust_rotation(-3), 1);
        assert_eq!(TestCorrectIndex::adjust_rotation(-8), 0);
        assert_eq!(TestCorrectIndex::adjust_rotation(-4), 0);
        assert_eq!(TestCorrectIndex::adjust_rotation(-0), 0);
        assert_eq!(TestCorrectIndex::adjust_rotation(-1), 3);
    }

    #[test]
    fn get_range() {
        assert_eq!(TestCorrectRanges::get_range(0, 5), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(TestCorrectRanges::get_range(5, 0), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(TestCorrectRanges::get_range(5, 5), vec![5]);
        assert_eq!(TestCorrectRanges::get_range(-5, -3), vec![-5, -4, -3]);
    }
}
