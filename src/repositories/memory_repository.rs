use crate::{entities::book::Book, interfaces::repository::Repository};

pub struct MemoryRepository {
    pub books: Vec<Book>,
}

impl Repository for MemoryRepository {
    fn add(&mut self, title: String) -> bool {
        let book = Book::new(title);
        self.books.push(book);

        true
    }
    fn list(&mut self) -> &Vec<Book> {
        &self.books
    }
    fn delete(&mut self) {}
}
