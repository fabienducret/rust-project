use crate::interfaces::{presenter::Presenter, repository::Repository};

pub struct LibraryServices {
    pub presenter: Box<dyn Presenter>,
    pub repository: Box<dyn Repository>,
}

impl LibraryServices {
    pub fn ask_title_and_store_book(&mut self) {
        let title = self.ask_title();
        self.store_book_and_display_result(title);
    }

    pub fn list_books(&mut self) {
        let books = self.repository.get_all();

        if books.len() > 0 {
            self.presenter.display_books(books);
        } else {
            self.presenter.print_text_green("No book stored.");
        }
    }

    pub fn ask_for_book_and_delete(&mut self) {
        let book_id = self.ask_for_book_to_delete();
        self.delete_book_and_display_result(book_id);
    }

    fn ask_title(&mut self) -> String {
        self.presenter.ask_for_book_title()
    }

    fn store_book_and_display_result(&mut self, title: String) {
        let is_book_created = self.repository.add(title.to_string());

        if is_book_created {
            self.presenter.print_text_green("Book correclty added !");
        } else {
            self.presenter.print_text_red("Error in book storing");
        }
    }

    fn ask_for_book_to_delete(&mut self) -> u32 {
        self.presenter.ask_for_book_to_delete()
    }

    fn delete_book_and_display_result(&mut self, book_id: u32) {
        self.repository.delete(book_id);
    }
}
