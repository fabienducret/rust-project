pub struct Book {
    title: String,
}

impl Book {
    pub fn new(title: String) -> Self {
        Book { title }
    }

    pub fn get_title(&self) -> String {
        return format!("{}", self.title);
    }
}
