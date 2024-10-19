mod users;
mod auth;

use std::io;

fn main() {
    loop {
        println!("Choose an option:");
        println!("[1] - Register");
        println!("[2] - Login");
        println!("[3] - Exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read input");

        match option.trim() {
            "1" => {users::create_user();},
            "2" => {auth::login();},
            "3" => {println!("Exiting the system...");break;},
             _  => {println!("Invalid option");}
        }
    }
}
