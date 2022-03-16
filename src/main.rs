mod entities;
mod interfaces;
mod presenters;
mod services;

use interfaces::library_services::ILibraryServices;
use interfaces::presenter::IPresenter;
use presenters::console_presenter::ConsolePresenter;
use services::library_services::LibraryServices;

fn main() {
    let mut presenter = ConsolePresenter {};
    let mut library_services = LibraryServices {
        presenter: Box::new(ConsolePresenter {}),
    };

    presenter.print_text_blue("\nWelcome on your Library Manager !\n".to_string());

    ask_for_action(&mut presenter, &mut library_services);
}

fn ask_for_action(presenter: &mut ConsolePresenter, library_services: &mut LibraryServices) {
    let mut action = 0;

    while action != 9 {
        presenter.print_text(
            "\n1 for adding \n2 for listing \n3 for deleting \n9 for leaving".to_string(),
        );

        action = get_action(presenter);

        match action {
            1 => library_services.add_book(),
            2 => library_services.list_book(),
            3 => library_services.delete_book(),
            9 => presenter.print_text_blue("\nBye ! :)".to_string()),
            _ => presenter.print_text_blue("\nInvalid action".to_string()),
        }
    }
}

fn get_action(presenter: &mut ConsolePresenter) -> u32 {
    let action_input = presenter.ask_for_line("Please enter your action :".to_string());

    let action = match action_input.trim().parse() {
        Ok(action) => action,
        Err(_) => 0,
    };

    return action;
}
