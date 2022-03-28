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
        let file = open_file();

        let data = "\n".to_string() + &title;
        let data = data.as_bytes();

        let mut buffer = BufWriter::new(file);
        buffer.write_all(data).expect("Unable to write data");

        true
    }

    fn get_all(&mut self) -> Vec<Book> {
        let file = get_file();
        let file_content = BufReader::new(file);

        file_content
            .lines()
            .map(|line| Book::new(line.unwrap()))
            .collect::<Vec<Book>>()
    }

    fn delete(&mut self, _book_id: u32) -> bool {
        true
    }
}

fn file_path() -> &'static str {
    "./data/books.csv"
}

fn open_file() -> File {
    OpenOptions::new()
        .append(true)
        .open(file_path())
        .expect("Unable to open file")
}

fn get_file() -> File {
    File::open(file_path()).expect("Unable to open file")
}
