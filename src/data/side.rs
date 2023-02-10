
pub struct Side {
    display_value: i32,
    id: i32,
}

impl Side {
    pub fn new(id:i32) -> Self {
        Side {
            display_value: id,
            id,
        }
    }
}