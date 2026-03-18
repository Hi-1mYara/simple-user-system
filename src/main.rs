use simple_user_system::{
    input::mkacc::*
};

fn main() {
    let start_uuid: u32 = 1000;
    let mut current_uuid = start_uuid;
    
    println!("Create a starter user:");

    let first_user = add_user(&mut current_uuid);
    let mut user_list = vec![first_user];
    
    let new_user = add_user(&mut current_uuid);

    user_list.push(new_user);

    for user in user_list {
        println!(
            "{:?}",
            user
        )
    }

    let mut total_user_count: u32 = current_uuid - 999;
    println!("User count: {total_user_count}");
}

