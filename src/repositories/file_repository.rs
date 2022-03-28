use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

use crate::{entities::book::Book, interfaces::repository::Repository};

pub struct FileRepository {}

impl Repository for FileRepository {
    fn new() -> Self {
        FileRepository {}
    }

    fn add(&mut self, title: String) -> bool {
        let file = OpenOptions::new()
            .append(true)
            .open("./data/books.csv")
            .expect("Unable to open file");

        let data = "\n".to_string() + &title;
        let data = data.as_bytes();

        let mut file = BufWriter::new(file);
        file.write_all(data).expect("Unable to write data");

        true
    }

    fn get_all(&mut self) -> Vec<Book> {
        let file_content = File::open("./data/books.csv").expect("Unable to open file");
        let file_content = BufReader::new(file_content);

        file_content
            .lines()
            .map(|line| Book::new(line.unwrap()))
            .collect::<Vec<Book>>()
    }

    fn delete(&mut self, _book_id: u32) -> bool {
        true
    }
}
