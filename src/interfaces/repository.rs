use crate::entities::book::Book;

pub trait Repository {
    fn add(&mut self, title: String) -> bool;
    fn list(&mut self) -> &Vec<Book>;
    fn delete(&mut self);
}
