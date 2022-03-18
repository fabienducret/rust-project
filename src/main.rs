mod entities;
mod interfaces;
mod presenters;
mod repositories;
mod services;

use entities::book::Book;
use interfaces::presenter::Presenter;
use presenters::console_presenter::ConsolePresenter;
use repositories::memory_repository::MemoryRepository;
use services::library_services::LibraryServices;

fn main() {
    let mut presenter = ConsolePresenter {};
    let mut library_services = LibraryServices {
        presenter: Box::new(ConsolePresenter {}),
        repository: Box::new(MemoryRepository {
            books: Vec::<Book>::new(),
        }),
    };

    presenter.print_text_blue("\nWelcome on your Library Manager !\n");

    ask_for_action(&mut presenter, &mut library_services);
}

fn ask_for_action(presenter: &mut ConsolePresenter, library_services: &mut LibraryServices) {
    let mut action = 0;

    while action != 9 {
        presenter.print_text("\n1 for adding \n2 for listing \n3 for deleting \n9 for leaving");

        action = presenter.ask_for_action();

        match action {
            1 => library_services.ask_title_and_store_book(),
            2 => library_services.list_books(),
            3 => library_services.delete_book(),
            9 => presenter.print_text_blue("\nBye ! :)"),
            _ => presenter.print_text_red("\nInvalid action"),
        }
    }
}
