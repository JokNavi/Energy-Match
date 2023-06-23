pub struct Game {
    pub current_game_values: Vec<u8>,
}

impl Game {
    pub fn view(&self, x: u8) -> Vec<Vec<u8>> {
        let mut new_list = Vec::new();
        for value in &self.current_game_values {
            let mut new_vec = self.current_game_values.clone();
            for item in &mut new_vec {
                *item = item.wrapping_add(x);
            }
            new_list.push(new_vec);
        }
        new_list
    }

    pub fn swipe_up(&mut self, x: u8) {
        for value in &mut self.current_game_values {
            *value = value.wrapping_add(x);
        }
    }
}