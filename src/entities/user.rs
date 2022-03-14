pub trait IUser {
    fn new(name: String) -> Self;
    fn get_name(&self) -> String;
}

pub struct User {
    name: String,
}

impl IUser for User {
    fn new(name: String) -> Self {
        User { name }
    }

    fn get_name(&self) -> String {
        return format!("{}", self.name);
    }
}
