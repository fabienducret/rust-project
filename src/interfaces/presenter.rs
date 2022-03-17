pub trait Presenter {
    fn print_text(&mut self, text: &str);
    fn print_text_blue(&mut self, text: &str);
    fn print_text_green(&mut self, text: &str);
    fn print_text_red(&mut self, text: &str);
    fn ask_for_line(&mut self, question: &str) -> String;
}
