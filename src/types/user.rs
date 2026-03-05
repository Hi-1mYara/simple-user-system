#[derive(Debug)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub uuid: u32,
    pub admin: bool,
}

impl User {
    pub fn new(username: String, uuid: u32, email: String) -> Self {
        Self  {
            active: true,
            username,
            email,
            uuid,
            admin: false,
        }
    }
}
