mod entities;
mod presenters;
use crate::presenters::console_presenter::Presenter;

fn main() {
    Presenter::print_text("\nWelcome on your Library Manager !\n".to_string());

    let mut action = 0;

    while action != 9 {
        action = ask_for_action();

        match action {
            1 => Presenter::print_text("1".to_string()),
            9 => Presenter::print_text("\nBye ! :)".to_string()),
            _ => Presenter::print_text("Invalid action".to_string()),
        }
    }
}

fn ask_for_action() -> i32 {
    Presenter::print_text(
        "\n1 for adding \n2 for listing \n3 for deleting \n9 for leaving".to_string(),
    );
    let action = Presenter::ask_for_line("Please enter your action :".to_string());

    return action.trim().parse().expect("invalid");
}
