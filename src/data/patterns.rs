use super::colors::ColoredText;

pub type TargetPatternItem = (i32, ColoredText);

pub struct TargetPattern{
    pattern: Vec<TargetPatternItem>
}

impl TargetPattern {
    pub fn new(min_value) -> Self {
         Self { pattern } 
        }

    

}