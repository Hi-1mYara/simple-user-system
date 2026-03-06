use simple_user_system::{
    types::user::User,
    input::mkacc::*
};

fn main() {
    let user1 = User::new(input_username(), 1000, input_email(), is_admin());
    println!(
        " active: {}\n username: {}\n email: {}\n uuid: {}\n admin: {}\n",
        user1.active,
        user1.username,
        user1.email,
        user1.uuid,
        user1.admin,
    );
}



