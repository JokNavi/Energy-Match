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
            //side: Self::select_side(rand::thread_rng().gen_range(1..crate::SIDE_AMOUNT)),
            side: Self::select_side(0),
        }
    }

    fn select_side(side_id: i32) -> Side {
        Side::new(side_id)
    }

    pub fn swipe_down(&mut self, amount: i32) {
        self.rotations = Self::correct_adjustment(self.rotations + amount);
        println!("Cube rotations: {}", self.rotations);
        self.side = Side::new(self.rotations);
    }

    pub fn swipe_up(&mut self, amount: i32) {
        self.rotations = Self::correct_adjustment(self.rotations - amount);
        println!("Cube rotations: {}", self.rotations);
        self.side = Side::new(self.rotations);
    }

    fn correct_adjustment(value: i32) -> i32 {
        let side_amount = crate::SIDE_AMOUNT;
        match value {
            _ if value < 0 => side_amount - (value.abs() - 1),
            _ if value > side_amount => value.abs() - 1,
            _ => value,
        }
    }
}
