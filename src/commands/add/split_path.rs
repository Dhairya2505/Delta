use std::{fs, path::{Path, PathBuf}};

pub fn split_path(path: &String, start: &str) {

    let original = Path::new(path);
    let components = original.parent();

    if let Some(parent) = components {
        let mut index_path = PathBuf::from(start);
        for comp in parent.components() {
            index_path.push(comp);
            if !index_path.exists() {
                fs::create_dir(&index_path).expect("Failed to create directory");
            }
        }
    }

    // let splitted: Vec<&str> = path.split("\\").collect();
    // let mut path_string = String::from(".\\.delta\\index");
    // for i in 0..splitted.len()-1 {
    //     path_string += &format!("\\{}",&splitted[i]);
    //     if !Path::new(&path_string).exists() {
    //         fs::create_dir(Path::new(&path_string)).expect("Failed to create directory !!!");
    //     }
    // }
}