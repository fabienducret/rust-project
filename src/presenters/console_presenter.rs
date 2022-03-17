use crate::interfaces::presenter::Presenter;
use std::io;

pub struct ConsolePresenter {}

impl Presenter for ConsolePresenter {
    fn print_text(&mut self, text: &str) {
        println!("{}", text);
    }

    fn print_text_blue(&mut self, text: &str) {
        self.print_text(format!("\x1b[34m{}\x1b[0m", text).as_str());
    }

    fn print_text_green(&mut self, text: &str) {
        self.print_text(format!("\x1b[32m{}\x1b[0m", text).as_str());
    }

    fn print_text_red(&mut self, text: &str) {
        self.print_text(format!("\x1b[31m{}\x1b[0m", text).as_str());
    }

    fn ask_for_line(&mut self, question: &str) -> String {
        self.print_text(question);
        let mut param = String::new();
        io::stdin()
            .read_line(&mut param)
            .expect("Failed to read line");

        return param;
    }
}
