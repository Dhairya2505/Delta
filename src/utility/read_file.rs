use std::{fs::File, io::{BufReader,BufRead}};

pub fn read_lines(path: &String) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect()
}