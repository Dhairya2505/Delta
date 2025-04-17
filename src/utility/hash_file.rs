use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufReader, Read};
use std::error::Error;

pub fn hash_file(path: &String) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    let hash_string = format!("{:x}", result);
    Ok(hash_string)
}