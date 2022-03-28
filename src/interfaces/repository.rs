use crate::entities::book::Book;

pub trait Repository {
    fn new() -> Self
    where
        Self: Sized;
    fn add(&mut self, title: String) -> bool;
    fn get_all(&mut self) -> Vec<Book>;
    fn delete(&mut self, book_id: u32) -> bool;
}
