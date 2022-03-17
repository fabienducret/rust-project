use crate::{entities::book::Book, interfaces::repository::Repository};

pub struct MemoryRepository {}

impl Repository for MemoryRepository {
    fn add(&mut self, title: String) -> Book {
        Book::new(title)
    }
    fn list(&mut self) {}
    fn delete(&mut self) {}
}
