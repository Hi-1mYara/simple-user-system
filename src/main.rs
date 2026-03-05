use users::{
    types::user::*,
    input::mkacc::*
};

fn main() {
    let mut user1 = User::new(input_username(), 1000, input_email());
    user1.admin = is_admin();
    dbg!(&user1);
}



