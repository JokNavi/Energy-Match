use super::indexes::{GenerateSlices, CorrectIndex};

pub struct TargetPattern {
    pub pattern: Vec<i32>,
}

impl CorrectIndex for TargetPattern {}
impl GenerateSlices for TargetPattern{}

impl TargetPattern {
    pub fn new(length: i32) -> Self {
        Self {
            pattern: Self::generate_slices(length),
        }
    }

    pub fn set_pattern(&mut self, new_pattern: Vec<i32>) {
        self.pattern = new_pattern.iter().map(|x| Self::adjust_rotation(*x) ).collect();
    }
}

#[cfg(test)]
mod TestTartgetPattern {
    use super::TargetPattern;

    #[test]
    fn set_pattern() {
        let mut target_pattern = TargetPattern::new(3);
        target_pattern.set_pattern(Vec::from([0, 1, 2, 3]));
        assert_eq!(target_pattern.pattern, Vec::from([0, 1, 2, 3]));

        target_pattern.set_pattern(Vec::from([4, 1, 2, -1]));
        assert_eq!(target_pattern.pattern, Vec::from([0, 1, 2, 3]));

        target_pattern.set_pattern(Vec::from([0, -5, 2, 3]));
        assert_eq!(target_pattern.pattern, Vec::from([0, 3, 2, 3]));
    }
}
