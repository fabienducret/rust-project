use crate::interfaces::{presenter::Presenter, repository::Repository};

pub struct LibraryServices {
    pub(crate) presenter: Box<dyn Presenter>,
    pub(crate) repository: Box<dyn Repository>,
}

impl LibraryServices {
    pub fn ask_title_and_store_book(&mut self) {
        self.presenter.print_text_blue("\nAdding a book section.\n");
        let title = self.ask_title();

        if !title.is_empty() {
            self.store_book(title);
        } else {
            self.presenter.print_text_red("Empty title.");
        }
    }

    fn ask_title(&mut self) -> String {
        let title = self.presenter.ask_for_line("Enter the title : ");
        return title.trim().to_string();
    }

    fn store_book(&mut self, title: String) {
        let is_created = self.repository.add(title.to_string());

        if is_created {
            self.presenter
                .print_text_green(&format!("Book \"{}\" correclty added !", title)[..]);
        }
    }

    pub fn list_books(&mut self) {
        self.presenter.print_text_blue("\nListing section.\n");

        let books = self.repository.list();

        if books.len() == 0 {
            self.presenter.print_text_green("No book stored.");
        } else {
            for (index, book) in books.iter().enumerate() {
                self.presenter
                    .print_text_green(&format!("{}. {}", index + 1, book.get_title())[..]);
            }
        }
    }

    pub fn delete_book(&mut self) {
        self.presenter.print_text_blue("\nDeleting section.\n");
    }
}
