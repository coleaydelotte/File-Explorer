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
        println!("Enter one of the following commands: [exit, in, up]");
        let mut input = String::new();
        user_input.read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input.trim() == "exit" {
            break;
        }
        else if input.trim() == "in" {
            functionality.clear_terminal();
            functionality.output_files(forward_dirs.clone(), false);
            let new_path: PathBuf = functionality.step_in(forward_dirs.clone());
            directory.set_pwd(new_path.clone());
            println!("Stepping In A Directory: {}", new_path.display());
            functionality.output_files(directory.find_forward_directories_and_files(), false);
            forward_dirs = directory.find_forward_directories();
        }
        else if input.trim() == "up" {
            println!("Stepping Up A Directory: {}", functionality.step_up());
            directory.set_pwd(PathBuf::from(functionality.get_pwd()));
            forward_dirs = directory.find_forward_directories();
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
