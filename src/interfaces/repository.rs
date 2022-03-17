use crate::entities::book::IBook;

pub trait Repository {
    fn add(&mut self, title: String) -> Box<dyn IBook>;
    fn list(&mut self);
    fn delete(&mut self);
}
