use crate::entities::book::Book;

pub trait Presenter {
    fn new() -> Self
    where
        Self: Sized;
    fn print_text(&self, text: &str);
    fn print_text_blue(&self, text: &str);
    fn print_text_green(&self, text: &str);
    fn print_text_red(&self, text: &str);
    fn ask_for_line(&self, question: &str) -> String;
    fn ask_for_action(&self) -> u32;
    fn ask_for_book_title(&self) -> String;
    fn ask_for_book_to_delete(&self) -> u32;
    fn display_books(&self, books: &Vec<Book>);
}
