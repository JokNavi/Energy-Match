use super::cube::Cube;

pub struct Bar {
    local_rotations: i32,
    index: i32,
    pub selected_cube: Cube,
    left_list: Vec<Cube>,
     right_list: Vec<Cube>,
}

pub enum Direction {
    Left,
    Right,
}

impl Bar {
    pub fn new() -> Self {
        Bar {
            local_rotations: 0,
            index: 0,
            selected_cube: Cube::new(0),
            left_list: Vec::new(),
            right_list: Vec::new(),
        }
    }
    
    pub fn swipe(&mut self, direction: Direction, amount: i32){
        let index = match direction {
            Direction::Left => self.index + amount,
            Direction::Right => self.index - amount,
        };
        self.index = index;
        println!("{}", index);
        if index >= 0 { self.swipe_left(index); }
        else { self.swipe_right(index.abs()); }
    }

    fn contains_cube(list: &[Cube], index: i32) -> Option<Cube> {
        return list.iter().find(|x| x.index == index).cloned();
    }

    fn swipe_left(&mut self, index: i32){
        if let Some(cube) = Self::contains_cube(&self.left_list, index) { self.selected_cube = cube; }
        else{ self.selected_cube = Cube::new(index); self.left_list.push(Cube::new(index)); }
        }
        
    fn swipe_right(&mut self, index: i32){
        if let Some(cube) = Self::contains_cube(&self.right_list, index) { self.selected_cube = cube; }
        else{ self.selected_cube = Cube::new(index); self.right_list.push(Cube::new(index)); }
        }
}

