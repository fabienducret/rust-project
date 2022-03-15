pub trait IPresenter {
    fn print_text(&mut self, text: String);
    fn print_text_blue(&mut self, text: String);
    fn print_text_green(&mut self, text: String);
    fn ask_for_line(&mut self, question: String) -> String;
}
