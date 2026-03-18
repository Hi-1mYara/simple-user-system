use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use simple_user_system::{
    input::mkacc::*, types::user::User
};

#[derive(Debug, Default)]
struct App {
    user_list: Vec<User>,
    uuid: u32,
    exit: bool
}

fn main() -> io::Result<()>{
    let mut app = App{user_list: Vec::new(), uuid: 1000, exit: false};

    app.run()?;
    Ok(())
}

impl App {
    pub fn run(&mut self) -> io::Result<()> {        
        while !self.exit {
            self.handle_event()?;
        }
        Ok(())
    }

    fn handle_event(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('e') => {
                println!("Make a user");
                self.user_list.push(add_user(&mut self.uuid))
            },
            KeyCode::Char('r') => todo!(),
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }
}

    // let start_uuid: u32 = 1000;
    // let mut current_uuid = start_uuid;
    
    // println!("Create a starter user:");

    // let first_user = add_user(&mut current_uuid);
    // let mut user_list = vec![first_user];
    
    // let new_user = add_user(&mut current_uuid);

    // user_list.push(new_user);

    // for user in user_list {
    //     println!(
    //         "{:?}",
    //         user
    //     )
    // }

    // let mut total_user_count: u32 = current_uuid - 999;
    // println!("User count: {total_user_count}");
