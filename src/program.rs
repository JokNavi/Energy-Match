use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

use crate::{data::bar::Bar, users::user::User};

pub struct Program {
    pub bar: Bar,
    pub user: User,
}

impl Program {
    pub fn new(file_name: Option<&str>) -> Self {
        Program {
            bar: Bar::new(),
            user: Self::get_user(file_name),
        }
    }

    pub fn get_user(file_name: Option<&str>) -> User {
        match file_name {
            None => User::new(),
            Some(file_name) => match Self::load_user(file_name) {
                Ok(user) => user,
                Err(err) => {
                    println!("User save failed. {err}");
                    User::new()
                },
            },
        }
    }

    pub fn load_user(file_name: &str) -> Result<User, io::Error> {
        let path = Path::new("src/users/user_files/").join(format!("{}.json", file_name));
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized: User = serde_json::from_str(&contents)?;
        Ok(deserialized)
    }
}
