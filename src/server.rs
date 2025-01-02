use crate::calendar::{date::Date, Calendar};
use crate::data::Data;
use crate::data_manager::json::Json;
use crate::data_manager::DataManager;
use crate::user::User;

use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Server {
    data: Data,
    date: Date,
    logged_user: Option<User>,
}

impl Server {
    pub fn new(year: u32, week: u32) -> Self {
        Server {
            data: Data::default(),
            date: Date::new(year, week),
            logged_user: None,
        }
    }
    pub fn start(&mut self) {
        println!("Server started !");
        loop {
            println!("Enter a command (help for help):");
            let command = self.read_input();
            let still_running = self.execute(&command);
            if !still_running {
                break;
            }
        }
        self.shut_down();
    }

    pub fn execute(&mut self, command: &str) -> bool {
        match command {
            "help" | "" => self.display_help(),
            "status" => self.display_status(),
            "register" => self.register(),
            "login" => self.login(),
            "logout" => self.logout(),
            "load" => self.load(),
            "save" => self.save(),
            "exit" => {
                self.shut_down();
                return false;
            }
            _ => println!("Unknown command"),
        }
        true
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

    pub fn load(&mut self) {
        let data_path = PathBuf::from("data/playerList.txt".to_owned());

        let data_manager = Json::new(data_path);
        self.data = match data_manager.load() {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                Data::default()
            }
        };
        println!("Data loaded");
    }

    fn save(&mut self) {
        let data_path = PathBuf::from("data/data.json".to_owned());
        let mut data_manager = Json::new(data_path);
        data_manager.save(self.data.clone()).unwrap();
        println!("Data saved");
    }

    fn read_input(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_owned()
    }

    fn display_help(&self) {
        println!("Available commands:");
        println!("help: Display this help message");
        println!("register: Register a new user");
        println!("login: Log in as a user");
        println!("logout: Log out the current user");
        println!("load: Load data from file");
        println!("save: Save data to file");
        println!("exit: Shut down the server");
    }

    fn display_status(&self) {
        match self.logged_user {
            None => println!("No user logged in"),
            Some(ref user) => println!("Logged in as: {}", user.username),
        }
        println!("Current date: {}", self.date);
        println!("Number of users: {}", self.data.users().len());
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
        match self.data.find_user(|user| user.username == username) {
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
