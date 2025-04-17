use std::fs;

pub fn create_folders() {

    fs::create_dir(".delta").expect("Failed to create folder !!!");

    fs::File::create(".delta/.commits").expect("Failed to create file !!!");
    fs::create_dir(".delta/stage").expect("Failed to create folder !!!");
    fs::create_dir(".delta/objects").expect("Failed to create folder !!!");
    fs::create_dir(".delta/index").expect("Failed to create folder !!!");
    fs::create_dir(".delta/commits").expect("Failed to create folder !!!");
    fs::create_dir(".delta/current state").expect("Failed to create folder !!!");

    println!("Delta repository initialized ...");

}