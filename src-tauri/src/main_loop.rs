use std::path::PathBuf;
use std::io::{stdin, stdout, Write};
use serde_json::value::Index;

use crate::directory;
use crate::functionality;
use crate::os_calls;

/**
 * This function is the main loop for the file explorer. It will
 * continue to prompt the user for input until the user types
 * "exit". The user can type "in" to step into a directory, "up"
 * to step up a directory, "ls" to list the files and directories
 * in the current directory, "pwd" to print the current directory,
 * and "cls" to clear the terminal.
 */
pub fn main_loop() {
    let os;
    #[cfg(target_os = "macos")] {
        os = "macos".to_string();
    }
    #[cfg(target_os = "windows")] {
        os = "windows".to_string();
    }
    //linux is a maybe
    // #[cfg(target_os = "linux")] {
    //     os = "linux".to_string();
    // }

    //TODO: get home directory in os_calls.
    let target_path;
    if os == "macos" {
        target_path = PathBuf::from("/Users");
    }
    else if os == "windows" {
        target_path = PathBuf::from("C:\\Users");
    }
    else {
        target_path = PathBuf::from("/home");
    }
    let mut directory = directory::Directory::new(target_path.clone());
    let mut functionality = functionality::Functionality::new(directory.get_pwd());
    let mut forward_dirs: Vec<String> = directory.find_forward_directories();
    println!("Welcome to the File Explorer!");
    stdout().flush().unwrap();
    let user_input = stdin();
    functionality.output_files(forward_dirs.clone(), false);
    loop {
        println!("Enter one of the following commands: [exit, in, up, ls, pwd, open, or cls]");
        let mut input = String::new();
        user_input.read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input.trim() == "exit" {
            break;
        }
        else if input.trim() == "cls" {
            functionality.clear_terminal();
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
        else if input.trim().contains("ls") {
            let forward_files = [directory.find_forward_files(), forward_dirs.clone()].concat();
            if input.trim().len() > 2 && input.trim() == "ls -s" {
                for i in 0..forward_files.len() {
                    let sizes = functionality.find_file_sizes(forward_files.clone());
                    print!("{}: {}", i + 1, forward_files[i]);
                    println!(" | Size: {} bytes", sizes[i]);
                }
            }
            else if input.trim() == "ls" {
                functionality.output_files(forward_files, true);
            }
        }
        else if input.trim().contains("open") {
            functionality.clear_terminal();
            let forward_files = directory.find_forward_files();
            let input_split: Vec<&str> = input.split_whitespace().collect();
            let index = match input_split[1].parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid Command: {}", input);
                    continue;
                }
            };
            let file_path = directory.get_pwd().join(&forward_files[(index - 1) as usize]);
            os_calls::open_file(file_path.to_str().unwrap());
        }
        else {
            println!("Invalid Command: {}", input);
        }
    }
    println!("Goodbye!");
}

/**
 * Passed a full filepath.
 * 
 * Returns the forward directories in the new directory.
 */
pub fn process_response_step_in(response: &String, pwd: String) -> (Vec<String>, String) {
    let mut directory = directory::Directory::new(PathBuf::from(pwd.trim()));
    let mut functionality = functionality::Functionality::new(directory.get_pwd());
    let forward_dirs = directory.find_forward_directories();
    let input = response.trim();
    let index: i32;
    if input.len() > 2 {
        functionality.clear_terminal();
        let input_split: Vec<&str> = input.split_whitespace().collect();
        index = match input_split[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Command: {}", input);
                return (directory.find_forward_directories(), directory.get_pwd().display().to_string());
            }
        };
        let new_path = functionality.step_in(forward_dirs.clone(), index);
        directory.set_pwd(new_path.clone());
        return (directory.find_forward_directories(), directory.get_pwd().display().to_string());
    }
    else {
        let new_path: PathBuf = functionality.step_in(forward_dirs.clone(), 0);
        directory.set_pwd(new_path.clone());
        return (directory.find_forward_directories(), directory.get_pwd().display().to_string());
    }
}

/**
 * Passed a full filepath.
 * 
 * Returns the parent directory's forward directories.
 */
pub fn process_response_step_up(path: &String) -> (Vec<String>, String) {
    let mut directory = directory::Directory::new(PathBuf::from(path.trim()));
    let mut functionality = functionality::Functionality::new(directory.get_pwd());
    directory.set_pwd(PathBuf::from(functionality.step_up()));
    return (directory.find_forward_directories(), directory.get_pwd().display().to_string());
}

/**
 * Passed a full filepath.
 * 
 * Returns the forward files and directories in the current directory.
 */
pub fn process_response_ls(response: &String) -> Vec<String> {
    let mut directory = directory::Directory::new(PathBuf::from(response.trim()));
    let forward_files = [directory.find_forward_directories(), directory.find_forward_files()].concat();
    return forward_files;
}

/**
 * Passed a full filepath.
 * 
 * Returns the current directory as a string.
 */
pub fn process_response_pwd(response: &String) -> String {
    let functionality = functionality::Functionality::new(PathBuf::from(response.trim()));
    return functionality.get_pwd().display().to_string();
}

/**
 * Passed a full filepath.
 * 
 * Opens the file at the given index in the current directory.
 */
pub fn process_response_open(response: &String) {
    let mut directory = directory::Directory::new(PathBuf::from(response.trim()));
    let forward_files = directory.find_forward_files();
    let input = response.trim();
    let input_split: Vec<&str> = input.split_whitespace().collect();
    let index = match input_split[1].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Command: {}", input);
            return;
        }
    };
    let file_path = directory.get_pwd().join(&forward_files[(index - 1) as usize]);
    os_calls::open_file(file_path.to_str().unwrap());
}