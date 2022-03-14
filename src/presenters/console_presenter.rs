use std::io;

pub struct Presenter {}

impl Presenter {
    pub fn print_text(text: String) {
        println!("{}", text);
    }

    pub fn ask_for_line(question: String) -> String {
        println!("{}", question);
        let mut param = String::new();
        io::stdin()
            .read_line(&mut param)
            .expect("Failed to read line");

        return param;
    }
}
