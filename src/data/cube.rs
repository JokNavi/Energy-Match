use  crate::Data::side::Side;

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


