use crate::entities::book::Book;

pub trait Repository {
    fn add(&mut self, title: String) -> Book;
    fn list(&mut self);
    fn delete(&mut self);
}
