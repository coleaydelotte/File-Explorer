use std::path::PathBuf;
use std::io::{stdin, stdout, Write};
use crate::directory;
use crate::functionality;

pub fn main_loop() {
    let target_path = PathBuf::from("C:\\Users");
    let mut directory = directory::Directory::new(target_path.clone());
    let mut functionality = functionality::Functionality::new(directory.get_pwd());
    let mut forward_dirs: Vec<String> = directory.find_forward_directories();
    println!("Welcome to the File Explorer!");
    stdout().flush().unwrap();
    let user_input = stdin();
    functionality.output_files(forward_dirs.clone(), false);
    loop {
        println!("Enter one of the following commands: [exit, in, up, ls, pwd, or cls]");
        let mut input = String::new();
        user_input.read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input.trim() == "exit" {
            break;
        }
        else if input.trim().contains("in") {
            let index: i32;
            if input.len() > 2 {
                functionality.clear_terminal();
                let input_split: Vec<&str> = input.split_whitespace().collect();
                index = match input_split[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid Command: {}", input);
                        continue;
                    }
                };
                let new_path = functionality.step_in(forward_dirs.clone(), index);
                directory.set_pwd(new_path.clone());
                println!("Stepping In A Directory: {}", new_path.display());
                let dirs = directory.find_forward_directories();
                functionality.output_files(dirs.clone(), false);
                forward_dirs = dirs;
            }
            else {
                functionality.clear_terminal();
                 functionality.output_files(forward_dirs.clone(), false);
                let new_path: PathBuf = functionality.step_in(forward_dirs.clone(), 0);
                directory.set_pwd(new_path.clone());
                functionality.clear_terminal();
                println!("Stepping In A Directory: {}", new_path.display());
                functionality.output_files(directory.find_forward_directories(), false);
                forward_dirs = directory.find_forward_directories();   
            }
        }
        else if input.trim() == "up" {
            functionality.clear_terminal();
            println!("Stepping Up A Directory: {}", functionality.step_up());
            directory.set_pwd(PathBuf::from(functionality.get_pwd()));
            forward_dirs = directory.find_forward_directories();
            functionality.output_files(forward_dirs.clone(), false);
        }
        else if input.trim() == "pwd" {
            println!("Current Directory: {}", functionality.get_pwd().display());
        }
        else if input.trim() == "ls" {
            functionality.output_files(forward_dirs.clone(), false);
        }
        else if input.trim() == "cls" {
            functionality.clear_terminal();
        }
        else {
            println!("Invalid Command: {}", input);
        }
    }
    println!("Goodbye!");
}