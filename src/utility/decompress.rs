use flate2::read::GzDecoder;
use std::{fs::File, io::{
    self,
    BufRead,
    BufReader
}};

pub fn decompress_file_lines(path: &String) -> io::Result<Vec<String>> {

    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let lines = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(lines)
}