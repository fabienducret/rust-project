pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> User {
        User { name }
    }

    pub fn get_name(&self) -> String {
        return format!("{}", self.name);
    }
}
