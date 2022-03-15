use crate::interfaces::presenter::IPresenter;
use std::io;

pub struct Presenter {}

impl IPresenter for Presenter {
    fn print_text(&mut self, text: String) {
        println!("{}", text);
    }

    fn print_text_blue(&mut self, text: String) {
        println!("\x1b[34m{}\x1b[0m", text);
    }

    fn print_text_green(&mut self, text: String) {
        println!("\x1b[32m{}\x1b[0m", text);
    }

    fn ask_for_line(&mut self, question: String) -> String {
        println!("{}", question);
        let mut param = String::new();
        io::stdin()
            .read_line(&mut param)
            .expect("Failed to read line");

        return param;
    }
}
