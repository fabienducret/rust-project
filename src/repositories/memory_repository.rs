use std::convert::TryInto;

use crate::{entities::book::Book, interfaces::repository::Repository};

pub struct MemoryRepository {
    pub books: Vec<Book>,
}

impl Repository for MemoryRepository {
    fn add(&mut self, title: String) -> bool {
        if !title.is_empty() {
            let book = Book::new(title);
            self.books.push(book);

            return true;
        } else {
            return false;
        }
    }
    fn get_all(&mut self) -> &Vec<Book> {
        &self.books
    }
    fn delete(&mut self, book_id: u32) {
        self.books.remove((book_id - 1).try_into().unwrap());
    }
}
