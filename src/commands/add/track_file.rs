use std::{
    fs::{
        File,
        OpenOptions
    },
    io::{
        self,
        BufRead,
        BufReader,
        Write
    },
    path::Path
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};

use crate::{commands::add::split_path::split_path, utility::{hash_file::hash_file, read_file::read_lines}};

fn decompress_file_lines(path: &String) -> io::Result<Vec<String>> {

    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let lines = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(lines)
}

fn compress_content_to_file(content: &str, output: &String) {
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

pub fn track_file (file_path: &String, ignore_files: &Vec<String>) {
    
    // check for file in .deltaignore
    let mut present = false;
    for i in 0..ignore_files.len() {
        if file_path.contains(&ignore_files[i]) {
            present = true;
            break;
        }
    }

    if !present {

        //track file
        let hash = hash_file(file_path).unwrap();
        let index_path_string = format!(".\\.delta\\index\\{}", file_path.trim_start_matches(".\\"));
        let index_path = Path::new(&index_path_string);

        if index_path.exists() && index_path.is_file() {
            let line = &read_lines(&index_path_string)[0];
            if &hash == line {
                println!("All changes in the file are tracked !!!");
                return;
            } else {
                track(&file_path, index_path_string, hash);
            }

        } else {
            split_path(&file_path, ".\\.delta\\index");
            File::create(&index_path_string).expect("Failed to create file !!!");
            track(&file_path, index_path_string, hash);
        }

        println!("+ {}", &file_path);
    }
}

fn track(file_path: &String, index_path_string: String, hash: String){
    // get the content from current state folder
    let current_path_string = format!("./.delta/current state/{}", file_path);
    let current_path = Path::new(&current_path_string);
    let original_lines = read_lines(&file_path);
    let mut content = String::new();
    if current_path.exists() && current_path.is_file() {

        // content of original file
        let mut i = 0;
        match decompress_file_lines(&current_path_string) {
            Ok(current_lines) => {
                if current_lines.len() == original_lines.len() {
                    let mut changed = false;
                    for line in current_lines {
                        if original_lines[i] != line {
                            let delta_line = format!("{} {}\n", i, original_lines[i]);
                            content+= &delta_line;
                            changed = true;
                        }
                        i+=1;
                    }
                    if !changed {
                        println!("All changes in the file are tracked !!!");
                        return;
                    }
                } else {
                    let mut i = 0;
                    let mut j = 0;
    
                    while i < original_lines.len() && j < current_lines.len() {
                        if original_lines[i] != current_lines[j] {
                            let delta_line = format!("{} {}\n", i, original_lines[i]);
                            content+= &delta_line;
                        }
                        i+=1;
                        j+=1;
                    }
                    if original_lines.len() > current_lines.len() {
                        while i<original_lines.len() {
                            let delta_line = format!("{} {}\n", i, original_lines[i]);
                            content+= &delta_line;
                            i+=1;
                        }
                    }
    
                }
            }
            Err(e) => {
                eprintln!("Failed to decompress file: {}", e);
            }
        }
        
    } else {
        // if the file is being tracked for the first time
        for i in 0..original_lines.len() {
            let delta_line = format!("{} {}\n", i, original_lines[i]);
            content+= &delta_line;
        }
    }
    
    // update the stage file content
    let stage_path_string = format!("./.delta/stage/{}",file_path);
    let stage_path = Path::new(&stage_path_string);
    
    if !stage_path.exists() || !stage_path.is_file() {
        split_path(&file_path, ".\\.delta\\stage");
        File::create(&stage_path).expect("Failed to create a file !!!");   
    }
    compress_content_to_file(&content, &stage_path_string);
    
    // update the hash in the index folder
    std::fs::write(&index_path_string, &hash).expect("Failed to write to the file !!!");
}


