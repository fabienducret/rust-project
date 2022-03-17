pub trait IBook {
    fn get_title(&self) -> String;
}

pub struct Book {
    pub title: String,
}

impl IBook for Book {
    fn get_title(&self) -> String {
        return format!("{}", self.title);
    }
}
