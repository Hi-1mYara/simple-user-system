/// A struct to store user data
/// 
/// # Examples
/// 
/// A function to make a user and print its contents
/// ```no_run
/// fn main() -> User {
///     let user = User::new("test", 1000, "test@example.com", false);
///     println!(
///         " active: {}\n username: {}\n email: {}\n uuid: {}\n admin: {}\n",
///         user.active,
///         user.username,
///         user.email,
///         user.uuid,
///         user.admin,
///     );
///     return user;
/// }
/// ```
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub uuid: u32,
    pub admin: bool,
}


impl User {
    /// A function to create a new instance of User
    /// 
    /// Activity status defaults to "true"
    /// 
    /// # Example
    /// ```no_run
    /// fn main() -> User {
    ///     let user = User::new("test", 1000, "test@example.com", false);
    ///     return user;
    /// }
    /// ```
    pub fn new(username: String, uuid: u32, email: String, admin: bool) -> Self {
        Self  {
            active: true,
            username,
            email,
            uuid,
            admin,
        }
    }
}
