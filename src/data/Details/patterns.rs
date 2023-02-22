use super::colors::{color_selector, ColoredText};
use core::fmt;
use rand::Rng;
use std::ops::Range;

pub struct TargetPattern {
    pattern: Vec<ColoredText>,
}

impl TargetPattern {
    pub fn new(range: Range<i32>) -> Self {
        Self {
            pattern: Self::generate_target_pattern(range),
        }
    }

    fn generate_target_pattern(range: Range<i32>) -> Vec<ColoredText> {
        range
            .into_iter()
            .map(|_| {
                ColoredText::new(
                    color_selector(rand::thread_rng().gen_range(1..=crate::SIDE_AMOUNT)).expect("value range is accounted for"),
                    "??".to_string(),
                )
            })
            .collect()
    }
}

impl fmt::Display for TargetPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( ")?;
        for text in &self.pattern {
            write!(f, "{} ", text)?;
        }
        write!(f, ")")
    }
}
