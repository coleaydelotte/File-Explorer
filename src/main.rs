use std::path::PathBuf;
use std::io::{stdin, stdout, Write};
mod directory;
mod functionality;

fn main() {
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
        //User can input in the following format: in num_of_directory || in then the user will be prompted for a number.
        if input.starts_with("in") {
            let split: Vec<&str> = input.split_whitespace().collect();
            let index = if split.len() > 1 {
                split[1].parse::<usize>().unwrap_or(0)
            } else {
                0
            };
            if &index - 1 < forward_dirs.len() {
                functionality.clear_terminal();
                functionality.output_files(forward_dirs.clone(), false);
                let new_path = functionality.step_in(forward_dirs.clone(), index as i32);
                println!("Stepping In A Directory: {}", new_path.display());
                directory.set_pwd(PathBuf::from(functionality.get_pwd()));
                forward_dirs = directory.find_forward_directories();
                functionality.output_files(forward_dirs.clone(), false);
            } else {
                println!("Invalid Directory Number: {}", index);
            }
        }
        //when user types up the program will step up a directory.
        if input.trim() == "up" {
            functionality.clear_terminal();
            println!("Stepping Up A Directory: {}", functionality.step_up());
            directory.set_pwd(PathBuf::from(functionality.get_pwd()));
            forward_dirs = directory.find_forward_directories();
            functionality.output_files(forward_dirs.clone(), false);
        }
        //pwd will print the current directory.
        if input.trim() == "pwd" {
            println!("Current Directory: {}", functionality.get_pwd().display());
        }
        //ls will list the directories in the current directory.
        if input.trim() == "ls" {
            functionality.output_files(forward_dirs.clone(), false);
        }
        //cls will clear the terminal.
        if input.trim() == "cls" {
            functionality.clear_terminal();
        }
        //if the user types an invalid command it will print out the invalid command, and prompt again.
        else {
            println!("Invalid Command: {}", input);
        }
    }
    println!("Goodbye!");
}
