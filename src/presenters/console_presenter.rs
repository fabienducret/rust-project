use crate::interfaces::ipresenter::IPresenter;
use std::io;

pub struct Presenter {}

impl IPresenter for Presenter {
    fn print_text(&mut self, text: String) {
        println!("{}", text);
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
