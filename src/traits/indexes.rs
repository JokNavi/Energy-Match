use crate::LEVEL_SIZE;
use rand::Rng;

use crate::game_logic::games::SIDE_AMOUNT;

pub enum RowIndexError {
    AboveMax,
    UnderZero,
    NonI32Fitting,
}

pub trait CorrectIndex {
    fn adjust_rotation(rotation: i32) -> i32 {
        if rotation >= 0 {
            return rotation.abs() % SIDE_AMOUNT;
        }
        (SIDE_AMOUNT - (rotation.abs() % SIDE_AMOUNT)) % SIDE_AMOUNT
    }

    fn validate_index<T>(index: T) -> Result<usize, String>
    where
        T: TryInto<i32> + TryInto<usize> + Copy,
    {
        let index: i32 = match index.try_into() {
            Err(_) => return Err("Can't be converted to i32".to_string()),
            Ok(value) => value,
        };

        match index {
            _ if index >= LEVEL_SIZE => Err("Index is out of bounds (too high)".to_string()),
            _ if index < 0 => Err("Index below 0".to_string()),
            _ => Ok(index as usize),
        }
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
    fn generate_slices(length: i32) -> Vec<i32> {
        (0..length)
            .map(|_| rand::thread_rng().gen_range(0..=SIDE_AMOUNT - 1))
            .collect()
    }
}

#[cfg(test)]
mod correct_index_tests {
    use crate::game_logic::games::LEVEL_SIZE;

    use super::{CorrectIndex, CorrectRanges, GenerateSlices};
    struct TestCorrectIndex;
    impl CorrectIndex for TestCorrectIndex {}
    struct TestCorrectRanges;
    impl CorrectRanges for TestCorrectRanges {}

    struct TestGenerateSlices;
    impl GenerateSlices for TestGenerateSlices {}

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

    #[test]
    fn generate_slices() {
        assert_eq!(TestGenerateSlices::generate_slices(5).len(), 5);
        assert_eq!(TestGenerateSlices::generate_slices(0).len(), 0);
    }

    #[test]
    fn validate_index() {
        let level_size_usize: usize = LEVEL_SIZE.try_into().unwrap();

        assert_eq!(TestCorrectIndex::validate_index(5 as i32), Ok(5 as usize));
        assert_eq!(TestCorrectIndex::validate_index(0 as i32), Ok(0 as usize));
        assert_eq!(
            TestCorrectIndex::validate_index(LEVEL_SIZE as i32),
            Err("Index is out of bounds (too high)".to_string())
        );
        assert_eq!(
            TestCorrectIndex::validate_index(-1 as i32),
            Err("Index below 0".to_string())
        );
        assert_eq!(
            TestCorrectIndex::validate_index(-3 as i32),
            Err("Index below 0".to_string())
        );

        assert_eq!(TestCorrectIndex::validate_index(5 as usize), Ok(5 as usize));
        assert_eq!(
            TestCorrectIndex::validate_index(level_size_usize - 1 as usize),
            Ok(level_size_usize - 1 as usize)
        );
        assert_eq!(TestCorrectIndex::validate_index(0 as usize), Ok(0 as usize));
        assert_eq!(
            TestCorrectIndex::validate_index(level_size_usize),
            Err("Index is out of bounds (too high)".to_string())
        );
    }
}
