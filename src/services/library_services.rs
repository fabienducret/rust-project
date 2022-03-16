use crate::interfaces::{library_services::ILibraryServices, presenter::Presenter};

pub struct LibraryServices {
    pub(crate) presenter: Box<dyn Presenter>,
}

impl ILibraryServices for LibraryServices {
    fn add_book(&mut self) {
        self.presenter
            .print_text_blue("\nadd book in service".to_string());
    }

    fn list_book(&mut self) {
        self.presenter
            .print_text_blue("\nlisting books in service".to_string());
    }

    fn delete_book(&mut self) {
        self.presenter
            .print_text_blue("\ndeleting book in service".to_string());
    }
}
