pub trait Presenter {
    fn print_text(&self, text: &str);
    fn print_text_blue(&self, text: &str);
    fn print_text_green(&self, text: &str);
    fn print_text_red(&self, text: &str);
    fn ask_for_line(&self, question: &str) -> String;
}
