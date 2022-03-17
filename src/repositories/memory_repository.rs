use crate::{
    entities::book::{Book, IBook},
    interfaces::repository::Repository,
};

pub struct MemoryRepository {}

impl Repository for MemoryRepository {
    fn add(&mut self, title: String) -> Box<dyn IBook> {
        Box::new(Book { title })
    }
    fn list(&mut self) {}
    fn delete(&mut self) {}
}
