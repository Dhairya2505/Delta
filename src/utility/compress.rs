use std::fs::OpenOptions;
use flate2::{write::GzEncoder, Compression};
use std::io::Write;

pub fn compress_content_to_file(content: &str, output: &String) {
    let output_file = OpenOptions::new()
        .write(true)
        .truncate(true) // overwrite existing content
        .open(output)
        .expect("Failed to open existing file");

    let mut encoder = GzEncoder::new(output_file, Compression::default());

    encoder
        .write_all(content.as_bytes())
        .expect("Failed to write compressed data");
    encoder.finish().expect("Failed to finish compression");
}