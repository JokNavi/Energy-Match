use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::Path,
    string,
};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub username: String,
    password: String,
    high_score: i32,
    scores: HashMap<String, i32>,
}

impl User {
    pub fn new() -> Self {
        User {
            username: User::get_username(),
            password: User::get_password(),
            high_score: 0,
            scores: HashMap::new(),
        }
    }

    fn ask_input(message: String) -> Result<String, io::Error> {
        print!("{message}");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_owned())
    }

    fn get_username() -> String {
        let mut input =
            Self::ask_input("Please enter your username: ".to_string()).unwrap_or_else(|err| {
                println!(
                "Error: Something went wrong when parsing your username. Please try again.\n{err}"
            );
                Self::get_username()
            });
        if input.is_empty() {
            println!("Error: Please enter a non-empty string.");
            input = Self::get_username();
        };
        input
    }

    fn get_password() -> String {
        let mut input =
            Self::ask_input("Please enter your password: ".to_string()).unwrap_or_else(|err| {
                println!(
                "Error: Something went wrong when parsing your password. Please try again.\n{err}"
            );
                Self::get_password()
            });
        if input.is_empty() {
            println!("Error: Please enter a non-empty string.");
            input = Self::get_password();
        };
        input
    }

    pub fn clear_users() {
        let path = "src/users/user_files";
        fs::remove_dir_all(path).expect("Path should only not exist for a milisecond");
        fs::create_dir(path).expect("Path was just deleted");
    }

    pub fn save_user(&self) -> Result<(), io::Error> {
        let serialized = serde_json::to_string(&self)?;
        let path = Path::new("src/users/user_files/").join(format!("{}.json", &self.username));
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
