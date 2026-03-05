use users::input::mkacc::*;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    uuid: u32,
    admin: bool,
}

impl User {
    fn new(username: String, uuid: u32, email: String) -> Self {
        Self  {
            active: true,
            username,
            email,
            uuid,
            admin: false,
        }
    }
}

fn main() {
    let mut user1 = User::new(input_username(), 1000, input_email());
    user1.admin = is_admin();
    dbg!(&user1);
}



