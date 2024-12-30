use crate::calendar::Calendar;
use crate::data::Data;
use crate::data_manager::text_file::TextFile;
use crate::data_manager::{self, DataManager};
use crate::user::User;

use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Server {
    data: Data,
    calendar: Calendar,
    logged_user: Option<User>,
}

impl Server {
    pub fn new(year: u32, week: u32) -> Self {
        Server {
            data: Data::default(),
            calendar: Calendar::new(year, week),
            logged_user: None,
        }
    }
    pub fn start(&mut self) {
        println!("Server started !");
        loop {
            let input = self.read_input();
            match input.as_str() {
                "help" => self.display_help(),
                "register" => self.register(),
                "login" => self.login(),
                "logout" => self.logout(),
                "load data" => self.load_data(),
                "save data" => self.save_data(),
                "exit" => break,
                _ => println!("Unknown command"),
            }
        }
        self.shut_down();
    }

    fn shut_down(&mut self) {
        if let Some(user) = &self.logged_user {
            println!("Logging out user {}", user.username);
            self.logout();
        }
        //println!("Saving data...");
        //self.save_data();
        println!("Server shutting down !");
    }

    pub fn load_data(&mut self) {
        let data_path = PathBuf::from("data/playerList.txt".to_owned());

        self.data = match TextFile::new(data_path) {
            Ok(data_manager) => {
                println!("Data manager created !");
                match data_manager.load() {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        Data::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                Data::default()
            }
        };
    }

    fn save_data(&mut self) {
        let data_path = PathBuf::from("data/playerList.txt".to_owned());
        let mut data_manager = TextFile::new(data_path).unwrap();
        data_manager.save(self.data.clone()).unwrap();
    }

    fn read_input(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_owned()
    }

    fn display_help(&self) {
        println!("Available commands:");
        println!("help: Display this help message");
        println!("exit: Shut down the server");
    }

    fn register(&mut self) {
        println!("Enter your username:");
        let username = self.read_input();
        println!("Enter your password:");
        let password = self.read_input();
        dbg!(&password);
        let password = match password.as_str() {
            "" => None,
            _ => Some(password),
        };
        let user = User::new(&username, password.as_deref());
        self.data.add_user(user);
    }

    fn login(&mut self) {
        println!("Enter your username:");
        let username = self.read_input();
        match self.data.get_user(&username) {
            Some(user) => {
                if user.has_password() {
                    println!("Enter your password:");
                    let password = self.read_input();
                    if !user.check_password(password.as_str()) {
                        println!("Wrong password");
                        return;
                    }
                }
                self.logged_user = Some(user.clone());
                println!("Logged in as {}", username);
            }
            None => println!("User not found"),
        }
    }

    fn logout(&mut self) {
        self.logged_user = None;
        println!("Logged out");
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[ignore]
    fn test_register() {
        let mut server = Server::default();
        server.register();
    }
}
