use crate::interfaces::presenter::Presenter;
use std::io;

pub struct ConsolePresenter {}

impl Presenter for ConsolePresenter {
    fn print_text(&self, text: &str) {
        println!("{}", text);
    }

    fn print_text_blue(&self, text: &str) {
        self.print_text(&format!("\x1b[34m{}\x1b[0m", text)[..]);
    }

    fn print_text_green(&self, text: &str) {
        self.print_text(&format!("\x1b[32m{}\x1b[0m", text)[..]);
    }

    fn print_text_red(&self, text: &str) {
        self.print_text(&format!("\x1b[31m{}\x1b[0m", text)[..]);
    }

    fn ask_for_line(&self, question: &str) -> String {
        self.print_text(question);
        let mut param = String::new();
        io::stdin()
            .read_line(&mut param)
            .expect("Failed to read line");

        return param;
    }
}
