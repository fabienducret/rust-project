use crate::{entities::book::Book, interfaces::presenter::Presenter};
use std::io;

pub struct ConsolePresenter {}

impl Presenter for ConsolePresenter {
    fn new() -> Self {
        ConsolePresenter {}
    }

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

    fn ask_for_action(&self) -> u32 {
        let action_input = self.ask_for_line("Please enter your action :");

        match action_input.trim().parse() {
            Ok(action) => action,
            Err(_) => 0,
        }
    }

    fn ask_for_book_title(&self) -> String {
        let title = self.ask_for_line("Enter the title : ");
        return title.trim().to_string();
    }

    fn ask_for_book_to_delete(&self) -> u32 {
        let book_id_input = self.ask_for_line("Enter the book id to delete :");

        match book_id_input.trim().parse() {
            Ok(id) => id,
            Err(_) => 0,
        }
    }

    fn display_books(&self, books: &Vec<Book>) {
        for (index, book) in books.iter().enumerate() {
            self.print_text_green(&format!("{}. {}", index + 1, book.get_title())[..]);
        }
    }
}
