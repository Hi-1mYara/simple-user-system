pub use std::io;

pub fn input_username() -> String {
    println!("Enter your username:");

    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    let username = username.trim_end().to_string();

    return username;
}

pub fn input_email() -> String {      // Asks for email
    println!("Enter your email:");
        
    let mut email = String::new();

    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");

    let email = email.trim_end().to_string();

    return email;
}

pub fn is_admin() -> bool {           // Asks for admin state
    loop {
        println!("Is this user an admin? (true or false)");
    
        let mut is_admin = String::new();

        io::stdin()
            .read_line(&mut is_admin)
            .expect("Failed to read line");

        let is_admin: bool = match is_admin.trim().parse() {    // converts string to bool
            Ok(bool) => bool,                                   // way easier than i thought,
            Err(_) => {                                         // the loop is so it doesn't just 
                println!("Not true or false");                  // stop entirely when it's wrong
                continue;
            }
        };
        return is_admin;
    }
}
