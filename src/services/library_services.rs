use crate::interfaces::{presenter::Presenter, repository::Repository};

pub struct LibraryServices {
    pub(crate) presenter: Box<dyn Presenter>,
    pub(crate) repository: Box<dyn Repository>,
}

impl LibraryServices {
    pub fn add_book(&mut self) {
        self.presenter.print_text_blue("\nAdding a book section.");
        let book_title = self.presenter.ask_for_line("Enter the title : ");

        self.repository.add(book_title);

        self.presenter.print_text_green("Book correclty added !");
    }

    pub fn list_books(&mut self) {
        self.presenter.print_text_blue("\nlisting books in service");
    }

    pub fn delete_book(&mut self) {
        self.presenter.print_text_blue("\ndeleting book in service");
    }
}
