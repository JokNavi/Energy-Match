#[derive(Clone)]
pub struct Side {
    pub display_value: i32,
    index: i32,
}

impl Side {
    pub fn new(index: i32) -> Self {
        Side {
            display_value: index,
            index,
        }
    }
}
