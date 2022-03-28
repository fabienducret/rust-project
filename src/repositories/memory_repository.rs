use crate::{entities::book::Book, interfaces::repository::Repository};

pub struct MemoryRepository {
    books: Vec<Book>,
}

impl Repository for MemoryRepository {
    fn new() -> Self {
        MemoryRepository {
            books: Vec::<Book>::new(),
        }
    }

    fn add(&mut self, title: String) -> bool {
        if !title.is_empty() {
            let book = Book::new(title);
            self.books.push(book);
            true
        } else {
            false
        }
    }

    fn get_all(&mut self) -> Vec<Book> {
        let mut books = Vec::<Book>::new();

        for db_book in &self.books {
            let book = Book::new(db_book.get_title());
            books.push(book);
        }

        books
    }

    fn delete(&mut self, book_id: u32) -> bool {
        let index_to_delete = (book_id - 1) as usize;
        let boot_to_delete = self.books.get(index_to_delete);

        match boot_to_delete {
            Some(_) => {
                self.books.remove(index_to_delete);
                true
            }
            _ => false,
        }
    }
}
