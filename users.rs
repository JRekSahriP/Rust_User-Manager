use std::io;

pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        return User {username, password}
    }
}

pub fn create_user() {
    println!("\nRegistering a new user:");

    let mut username = String::new();
    println!("Enter username:");
    io::stdin().read_line(&mut username).expect("Failed to read input");
    let username = username.trim().to_string();

    let mut password = String::new();
    println!("Enter password:");
    io::stdin().read_line(&mut password).expect("Failed to read input");
    let password = password.trim().to_string();

    let new_user = User::new(username, password);
    save_user(new_user);
}

fn save_user(user: User) {
    println!("User '{}' registered successfully!", user.username);
}
