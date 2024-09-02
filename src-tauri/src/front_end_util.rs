use crate::main_loop;
use crate::functionality;
use crate::directory;
use crate::os_calls;
use std::path::PathBuf;

/**
 * Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
 * 
 * Creating Tauri handlers for the front end to call.
 */
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn step_in(response: String, pwd: String) -> (Vec<String>, String) {
    let forward_dirs;
    let new_path;
    (forward_dirs, new_path) = main_loop::process_response_step_in(&response, pwd);
    return (forward_dirs, new_path.to_string());
}

#[tauri::command]
pub fn step_up(path: String) -> (Vec<String>, String) {
    let forward_dirs;
    let new_path;
    (forward_dirs, new_path) = main_loop::process_response_step_up(&path);
    return (forward_dirs, new_path.to_string());
}

#[tauri::command]
pub fn open_file(index: i32, path: String) {
    main_loop::process_response_open(index, &path);
}

#[tauri::command]
pub fn output_files_as_vector(path: String, print_files: bool) -> Vec<String> {
    let mut directory = directory::Directory::new(PathBuf::from(path.trim()));
    let mut functionality = functionality::Functionality::new(directory.get_pwd());
    return functionality.output_files_as_vector(directory.find_forward_files(), print_files);
}

#[tauri::command]
pub fn get_os() -> String {
    return os_calls::get_os();
}