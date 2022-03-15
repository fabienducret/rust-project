use crate::interfaces::{library_services::ILibraryServices, presenter::IPresenter};

pub struct LibraryServices {
    pub(crate) presenter: Box<dyn IPresenter>,
}

impl ILibraryServices for LibraryServices {
    fn add_book(&mut self) {
        self.presenter
            .print_text_blue("add book in service".to_string());
    }

    fn list_book(&mut self) {
        self.presenter
            .print_text_blue("listing books in service".to_string());
    }

    fn delete_book(&mut self) {
        self.presenter
            .print_text_blue("deleting book in service".to_string());
    }
}