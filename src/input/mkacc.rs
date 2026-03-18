use std::io;
use crate::types::user::*;

pub fn add_user(user_count: &mut u32) -> User {
    *user_count += 1;
    
    let user = User::new(input_string("username"), *user_count, input_string("email"), is_admin());
    
    return user
}

pub fn input_string(input_type: &str) -> String {      // Asks for email
    println!("Enter your {input_type}:");
        
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");

    let string = string.trim().to_string();

    string
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

