use crate::interfaces::{ilibrary_services::ILibraryServices, ipresenter::IPresenter};

pub struct LibraryServices {
    pub(crate) presenter: Box<dyn IPresenter>,
}

impl ILibraryServices for LibraryServices {
    fn add_book(&mut self) {
        self.presenter.print_text("add book in service".to_string());
    }
}
