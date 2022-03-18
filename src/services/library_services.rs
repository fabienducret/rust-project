use crate::interfaces::{presenter::Presenter, repository::Repository};

pub struct LibraryServices {
    pub presenter: Box<dyn Presenter>,
    pub repository: Box<dyn Repository>,
}

impl LibraryServices {
    pub fn ask_title_and_store_book(&mut self) {
        self.presenter.print_text_blue("\nAdding a book section.\n");
        let title = self.presenter.ask_for_book_title();

        if !title.is_empty() {
            self.store_book_and_display_result(title);
        } else {
            self.presenter.print_text_red("Empty title.");
        }
    }

    fn store_book_and_display_result(&mut self, title: String) {
        let is_created = self.repository.add(title.to_string());

        if is_created {
            self.presenter
                .print_text_green(&format!("Book \"{}\" correclty added !", title)[..]);
        } else {
            self.presenter.print_text_red("Error in book storing");
        }
    }

    pub fn list_books(&mut self) {
        self.presenter.print_text_blue("\nListing section.\n");
        let books = self.repository.get_all();

        if books.len() > 0 {
            self.presenter.display_books(books);
        } else {
            self.presenter.print_text_green("No book stored.");
        }
    }

    pub fn delete_book(&mut self) {
        self.presenter.print_text_blue("\nDeleting section.\n");
        let book_id = self.presenter.ask_for_book_to_delete();

        self.repository.delete(book_id);
    }
}
