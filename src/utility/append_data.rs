use std::fs::OpenOptions;
use std::io::Write;

pub fn append(path: &str, content: &String) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Failed to open file");

    writeln!(file, "\n{}", content).expect("Failed to write");
}