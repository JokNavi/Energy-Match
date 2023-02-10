pub struct Cube {
    rotations: i32,
    side: Side,
}

impl Cube {
    pub fn new(rotations:i32) -> Self {
        Cube {
            rotations,
            side: Self::select_side(rotations%crate::SIDE_AMOUNT)
        }
    }

    fn select_side(side_id: i32) -> Side{
        Side::new(side_id)
    }
}

pub struct Side {
    display_value: i32,
    id: i32,
}

impl Side {
    fn new(id:i32) -> Self {
        Side {
            display_value: id,
            id,
        }
    }
}

