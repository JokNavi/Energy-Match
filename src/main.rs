use crate::program::Program;

mod data;
mod display;
mod program;
mod users;
const SIDE_AMOUNT: i32 = 4;

fn main() {
    println!("Creating new user...");
    let mut program = Program::new(None);
    match program.user.save_user() {
        Ok(_) => println!("User saved successfully!"),
        Err(err) => println!("User save failed. {err}"),
    };
    
}
