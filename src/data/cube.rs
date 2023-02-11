use crate::data::side::Side;
use rand::Rng;

#[derive(Clone)]
pub struct Cube {
    pub index: i32,
    pub rotations: i32,
    pub side: Side,
}

impl Cube {
    pub fn new(index: i32) -> Self {
        Cube {
            index,
            rotations: 0,
            side: Self::select_side(rand::thread_rng().gen_range(0..crate::SIDE_AMOUNT)),
        }
    }

    fn select_side(side_id: i32) -> Side {
        Side::new(side_id)
    }

    pub fn swipe_down(&mut self) {
        self.rotations = Self::correct_adjustment(self.rotations + 1)
    }

    pub fn swipe_up(&mut self) {
        self.rotations = Self::correct_adjustment(self.rotations - 1)
    }

    fn correct_adjustment(value: i32) -> i32 {
        let side_amount = crate::SIDE_AMOUNT;
        match value{
            _ if value < side_amount => side_amount,
            _ if value > side_amount => 0,
            _ => value,
        }
    }

    
}

