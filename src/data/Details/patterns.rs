use rand::Rng;
use std::ops::Range;

use super::colors::{color_selector, ColoredText};

pub type TargetPatternItem = (i32, ColoredText);

pub struct TargetPattern {
    pattern: Vec<TargetPatternItem>,
}

impl TargetPattern {
    pub fn new(range: Range<i32>) -> Self {
        Self {
            pattern: Self::generate_target_pattern(range),
        }
    }

    fn generate_target_pattern(range: Range<i32>) -> Vec<TargetPatternItem> {
        let mut target_pattern: Vec<TargetPatternItem> = Vec::new();
        for _ in range {
            let value = rand::thread_rng().gen_range(1..=crate::SIDE_AMOUNT);
            target_pattern.push((
                value,
                ColoredText::new(color_selector(value), value.to_string()),
            ))
        }
        return target_pattern;
    }
}
