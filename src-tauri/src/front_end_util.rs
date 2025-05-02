use crate::main_loop;
use crate::functionality;
use crate::directory;
use crate::os_calls;
use std::path::PathBuf;

// use std::io::Write;
// potential output to files

/**
 * Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
 * 
 * Creating Tauri handlers for the front end to call.
 */
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/**
 * Response is the index of file to step into
 * 
 * pwd is the full filepath to be mutated.
 */
#[tauri::command]
pub fn step_in(response: String, pwd: String) -> String {
    let new_path = main_loop::process_response_step_in(&response, pwd);
    return new_path.to_string();
}

/**
 * Response is the index of file to step into
 * 
 * pwd is the full filepath to be mutated.
 */
#[tauri::command]
pub fn step_up(path: String) -> String {
    let new_path = main_loop::process_response_step_up(&path);
    return new_path.to_string();
}

/**
 * Opens a file in the users OS.
 */
#[tauri::command]
pub fn open_file(index: i32, path: String) {
    main_loop::process_response_open(index, &path);
}

/**
 * This function is used to output the files and directories in the current
 * directory to the console and to a file. It takes a vector of strings
 * representing the files and directories in the current directory and a
 * boolean value indicating whether to print the files and directories
 * to the console or not. It returns a vector of strings representing
 * the files and directories in the current directory.
 */
#[tauri::command]
pub fn output_files_as_vector(path: String, print_files: bool) -> Vec<String> {
    let path_buf = PathBuf::from(path.trim());
    let mut directory = directory::Directory::new(path_buf);
    let mut functionality = functionality::Functionality::new(directory.get_pwd());
    let forward_files = directory.find_forward_directories_and_files();

    return functionality.output_files_as_vector(forward_files, print_files);
}

/**
 * This function returns the current user's operating system.
 */
#[tauri::command]
pub fn get_os() -> String {
    return os_calls::get_os();
}

/**
 * This function formats a path for the current operating system.
 * It takes a string representing the path and returns a string
 * representing the formatted path.
 */
#[tauri::command]
pub fn format_path_for_windows(path: String) -> String {
    let functionality = functionality::Functionality::new_empty();
    return functionality.format_path_for_windows(path.clone());
}

/**
 * This function lists the files and directories in the current directory.
 * It takes a string representing the path and returns a vector of strings
 * representing the files and directories in the current directory.
 */
#[tauri::command]
pub fn ls(path: String) -> Vec<String> {
    return main_loop::process_response_ls(&path);
}
