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
    add::{
        split_path::split_path,
        track_file::track_file
    },
    config::{
        check_username::check_username,
        verify_password::verify_password
    },
    init::create_delta_folders::create_folders, push::{aws::aws_fn, s3_fn::s3_fn}
};

mod utility;
use utility::{
    append_data::append,
    compress::compress_content_to_file,
    decompress::decompress_file_lines,
    generate_random_id::generate_random_id,
    read_file::read_lines
};

use walkdir::WalkDir;

use sha2::{Sha256, Digest};

use chrono::prelude::*;

use chrono_tz::Asia::Kolkata;

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

            let path = Path::new("/usr/local/bin/.config");
            if !path.exists() || !path.is_file() {
                File::create(&path).expect("Failed to create .config !!!");
            }

            let username = &args[3];

            // check for username in the db
            match check_username(username) {
                Ok(true) => {

                    // update the .config file with username
                    let lines = read_lines(&String::from("/usr/local/bin/.config"));
                    fs::write("/usr/local/bin/.config", username).expect("Failed to write !!!");
                    if lines.len() == 2 && &lines[0] == username {
                        append("/usr/local/bin/.config", &lines[1]);
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
            
            let path = Path::new("/usr/local/bin/.config");
            if !path.exists() || !path.is_file() {
                File::create("/usr/local/bin/.config").expect("Failed to create .config !!!");
            }
            let lines = read_lines(&String::from("/usr/local/bin/.config"));
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

        // check for the username
        let path = Path::new("/usr/local/bin/.config");
        if !path.exists() || !path.is_file() {
            println!("Firstly authenticate yourself !!!");
            println!("Command -> delta config user.name <username>");
            println!("Command -> delta config user.password <password>");
            return;
        }
        let lines = read_lines(&String::from("/usr/local/bin/.config"));
        if lines.len() == 0 {
            println!("Firstly authenticate yourself !!!");
            println!("Command -> delta config user.name <username>");
            println!("Command -> delta config user.password <password>");
            return;
        }
        
        let username = &lines[0];
        
        let mut blob_hash: Vec<String> = Vec::new();
        
        let name = &args[2];
        
        // get the content of each file from stage
        for entry in WalkDir::new("./.delta/stage")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file()) {

                let path = entry.path();
                let path_string = format!("{}", &path.display());

                match decompress_file_lines(&path_string) {
                    Ok(lines) => {

                        // extract the content of the file from stage folder
                        let mut content = String::new();
                        content = content + &path_string[14..] + "\n";
                        for line in lines {
                            content = content + &line + "\n";
                        }

                        // add to current state folder
                        let original_path = format!(".{}",&path_string[14..]);
                        
                        let original_content = fs::read_to_string(&original_path).expect("Failed to read file !!!");
                        let current_path_string = format!("./.delta/current state/{}", &path_string[14..]);
                        
                        split_path(&original_path, "./.delta/current state");
                        File::create(&current_path_string).expect("Failed to create file !!!");
                        compress_content_to_file(&original_content, &current_path_string);

                        // get the hash from the index folder
                        let index_path = format!("./.delta/index/{}", &path_string[14..]);
                        let hash = fs::read_to_string(&index_path).expect("Failed to read file !!!");
                        let hash_string = format!("{}", hash);
                        blob_hash.push(hash_string);


                        // split the hash for blob storage
                        let first = &hash[0..2];
                        let second = &hash[2..];

                        let mut object_path_string = format!("./.delta/objects/{}", &first);
                        let object_path = Path::new(&object_path_string);

                        if !object_path.exists() || !object_path.is_dir() {
                            fs::create_dir(&object_path_string).expect("Failed to create directory !!!");
                        }

                        // store the blob
                        object_path_string = format!("{}/{}", &object_path_string, &second);
                        File::create(&object_path_string).expect("Failed to create a file !!!");
                        compress_content_to_file(&content, &object_path_string);

                    }
                    Err(e) => {
                        eprintln!("Failed to decompress file: {}", e);
                    }
                }   

            }

            // commit id
            let id = generate_random_id();
            
            // commit time
            let utc_now = Utc::now();
            let ist_now = utc_now.with_timezone(&Kolkata);
            let timestamp = ist_now.format("%Y-%m-%d %H:%M:%S").to_string();
            
            // commit content
            let mut commit_content = String::from(id);
            commit_content = commit_content + "\n" + &name;
            commit_content = commit_content + "\n" + &username;
            commit_content = commit_content + "\n" + &timestamp;
            for hash in blob_hash {
                commit_content = commit_content + "\n" + &hash;
            }

            // hash the commit content
            let mut hasher = Sha256::new();
            hasher.update(&commit_content);
            let result = hasher.finalize();
            let commit_hash = hex::encode(result);

            // compress and save it to a file
            let commit_path = format!("./.delta/commits/{}", &commit_hash);
            File::create(&commit_path).expect("Failed to create a file !!!");
            compress_content_to_file(&commit_content, &commit_path);

            append("./.delta/.commits", &commit_hash);


            // remove all the files from stage folder
            let dir_path = Path::new("./.delta/stage");
            if dir_path.exists() && dir_path.is_dir() {
                for entry in fs::read_dir(dir_path).expect("Failed to read directory") {
                    let entry = entry.expect("Failed to get entry");
                    let path = entry.path();
                    if path.is_dir() {
                        fs::remove_dir_all(&path).expect("Failed to remove directory");
                    } else {
                        fs::remove_file(&path).expect("Failed to remove file");
                    }
                }
            }


    // push the changes to delta repo
    } else if command == "push" {

        let path = Path::new("/usr/local/bin/.config");
        if !path.exists() || !path.is_file() {
            println!("Firstly authenticate yourself !!!");
            println!("Command -> delta config user.name <username>");
            println!("Command -> delta config user.password <password>");
            return;
        }
        let lines = read_lines(&String::from("/usr/local/bin/.config"));
        if lines.len() != 2 {
            println!("Firstly authenticate yourself !!!");
            println!("Command -> delta config user.name <username>");
            println!("Command -> delta config user.password <password>");
            return;
        }

        let username = &lines[0];

        let repo_path = Path::new("/usr/local/bin/.repo");
        let mut repo_id = String::new();
        let mut repo_name = String::new();

        if repo_path.exists() && repo_path.is_file() {
            let lines = read_lines(&String::from("/usr/local/bin/.repo"));
    
            repo_name = lines[0].clone();
            repo_id = lines[1].clone();
        } else {
            return;
        }


        let private_path_string = format!("./.private");
        let private_path = Path::new(&private_path_string);

        if private_path.exists() && private_path.is_file() {

            return;
        }

        let protected_path_string = format!("./.protected");
        let protected_path = Path::new(&protected_path_string);

        if protected_path.exists() && protected_path.is_file() {
            let mut files: Vec<(String, bool)> = Vec::new();
            // get the files from protected path
            let protected_files = read_lines(&protected_path_string);
            for entry in WalkDir::new("./.delta")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file()) {

                let path = entry.path();
                let path_string = format!("{}", &path.display());
                
                let mut is_private = false;
                for file in &protected_files {
                    if path_string.contains(file) {
                        is_private = true;
                        break;
                    }
                }

                dotenvy::dotenv().ok();
                // save in dynamoDB and S3
                files.push((path_string, is_private));
            }   

            aws_fn(&username, repo_id.clone(), &files, &repo_name);
            
            
            let _ = s3_fn(&files, repo_id.clone());

            return;
        }






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
