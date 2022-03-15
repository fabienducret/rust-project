mod entities;
mod interfaces;
mod presenters;
mod services;

use interfaces::ilibrary_services::ILibraryServices;
use interfaces::ipresenter::IPresenter;
use presenters::console_presenter::Presenter;
use services::library_services::LibraryServices;

fn main() {
    let mut presenter = Presenter {};
    let mut library_services = LibraryServices {
        presenter: Box::new(Presenter {}),
    };

    presenter.print_text("\nWelcome on your Library Manager !\n".to_string());

    let mut action = 0;

    while action != 9 {
        action = ask_for_action();

        match action {
            1 => library_services.add_book(),
            9 => presenter.print_text("\nBye ! :)".to_string()),
            _ => presenter.print_text("Invalid action".to_string()),
        }
    }
}

fn ask_for_action() -> i32 {
    let mut presenter = Presenter {};

    presenter
        .print_text("\n1 for adding \n2 for listing \n3 for deleting \n9 for leaving".to_string());
    let action = presenter.ask_for_line("Please enter your action :".to_string());

    return action.trim().parse().expect("invalid");
}
