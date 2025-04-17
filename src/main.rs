use std::{
    env,
    fs::{
        self,
        File,
    },
    path::Path
};

mod commands;
use commands::{
    add::track_file::track_file,
    config::{
        check_username::check_username,
        verify_password::verify_password
    },
    init::create_delta_folders::create_folders
};

mod utility;
use utility::{
    append_data::append,
    read_file::read_lines
};

use walkdir::WalkDir;

fn main() {

    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];

    if command == "config" {
        if args.len() != 4 {
            println!("Invalid arguements !!");
            return;
        }

        // setting username
        if args[2] == "user.name" {

            let path = Path::new("C:/delta/.config");
            if !path.exists() || !path.is_file() {
                File::create(&path).expect("Failed to create .config !!!");
            }

            let username = &args[3];

            // check for username in the db
            match check_username(username) {
                Ok(true) => {

                    // update the .config file with username
                    let lines = read_lines(&String::from("C:/delta/.config"));
                    fs::write("C:/delta/.config", username).expect("Failed to write !!!");
                    if lines.len() == 2 && &lines[0] == username {
                        append("C:/delta/.config", &lines[1]);
                    } else {
                        println!("Add password");
                        println!("Command -> delta config user.password <password>");
                    }

                },
                Ok(false) => {
                    println!("User does not exist !!!");
                },
                Err(e) => {
                    eprintln!("Error checking username: {}", e);
                }
            }

        // setting password
        } else if args[2] == "user.password" {
            
            let path = Path::new("C:/tools/.config");
            if !path.exists() || !path.is_file() {
                File::create("C:/tools/.config").expect("Failed to create .config !!!");
            }
            let lines = read_lines(&String::from("C:/delta/.config"));
            if lines.len() > 0 {
                let username = &lines[0];
                let password = &args[3];
                
                if verify_password(&username, &password).unwrap() {
                    println!("Authentication successful !!!");
                } else {
                    println!("username or password incorrect !!!");
                }
            }

        // invalid argument 
        } else {
            println!("Invalid arguements !!");
        }

    // init command
    } else if command == "init" {
        if args.len() != 2 {
            println!("Command -> delta init");
            return;
        }

        // check for already initialized repo


        // initialize delta repository
        create_folders();


    // add command
    } else if command == "add" {
        if args.len() != 3 {
            println!("Command -> delta add <file name>");
            return;
        }

        // check for repository initialization
        let delta_path = Path::new("./.delta");
        if !delta_path.exists() || !delta_path.is_dir() {
            println!("Initialize a delta repository first !!!");
            println!("Command -> delta init");
            return;
        }

        // .deltaignore file and extract all the files
        let file = Path::new(".deltaignore");
        let mut ignore_files: Vec<String> = Vec::new();

        if file.exists() && file.is_file() {
            ignore_files = fs::read_to_string(".deltaignore")
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect();
        }

        // delta add .
        if args[2] == "." {
            
            for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file()) {

                let path = entry.path();
                let path_string = format!("{}", &path.display());
                track_file(&path_string, &ignore_files);

            }

        // delta add <file name>
        } else {

            let path = &args[2];
            if !Path::new(&path).exists() || !Path::new(&path).is_file() {
                println!("File does not exist !!!");
                return;
            }

            let file_path: &String = &args[2];

            track_file(file_path, &ignore_files);

        }

    // commit the changes
    } else if command == "commit" {


    // push the changes to delta repo
    } else if command == "push" {


    // pull a repo from server
    } else if command == "pull" {
        

    // help guide
    } else if command == "help" {
    

    // wrong command
    } else {
        println!("Enter a valid delta command !!!");
        println!("Command -> delta help");
    }


}
