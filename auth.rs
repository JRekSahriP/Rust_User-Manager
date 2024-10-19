use std::io;

pub fn login() {
    println!("\nLogin screen:");

    let mut username = String::new();
    println!("Username:");
    io::stdin().read_line(&mut username).expect("Failed to read input");
    let username = username.trim().to_string();

    let mut password = String::new();
    println!("Password:");
    io::stdin().read_line(&mut password).expect("Failed to read input");
    let password = password.trim().to_string();

    if authenticate(username, password) {
        println!("Login successful!");
    } else {
        println!("Incorrect username or password!");
    }
}

fn authenticate(username: String, password: String) -> bool {
    let example_user = "admin";
    let example_pass = "123";

    return username == example_user && password == example_pass;
}
