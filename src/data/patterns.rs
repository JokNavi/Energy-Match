use super::colors::ColoredText;

type TargetPatternItem = (i32, ColoredText);

pub struct TargetPattern{
    pattern: Vec<TargetPatternItem>
}