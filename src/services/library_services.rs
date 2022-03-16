use crate::interfaces::presenter::Presenter;

pub struct LibraryServices {
    pub(crate) presenter: Box<dyn Presenter>,
}

impl LibraryServices {
    pub fn add_book(&mut self) {
        self.presenter
            .print_text_blue("\nadd book in service".to_string());
    }

    pub fn list_book(&mut self) {
        self.presenter
            .print_text_blue("\nlisting books in service".to_string());
    }

    pub fn delete_book(&mut self) {
        self.presenter
            .print_text_blue("\ndeleting book in service".to_string());
    }
}
