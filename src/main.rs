mod entities;
mod presenters;
use crate::presenters::console_presenter::Presenter;

fn main() {
    Presenter::print_text("Welcome on your Library Manager !".to_string());

    let name = Presenter::ask_for_line("What is your name ?".to_string());

    Presenter::print_text("Hello, ".to_string() + &name);
}
