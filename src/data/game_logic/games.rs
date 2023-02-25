use crate::data::{details::patterns::TargetPattern, visual_objects::rows::Row};

pub const SIDE_AMOUNT: i32 = 4;

struct Game {
    row: Row,
    moves_done: i32,
    target_pattern: TargetPattern,
}
