// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

mod main_loop;
mod functionality;
mod directory;
mod os_calls;

/**
 * Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
 * 
 * Creating Tauri handlers for the front end to call.
 */
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn process_response_step_in(index: i32, pwd: String) -> (Vec<String>, String) {
    let mut directory = directory::Directory::new(PathBuf::from(pwd.clone()));
    let mut functionality = functionality::Functionality::new(directory.get_pwd());
    let forward_dirs: Vec<String> = directory.find_forward_directories();
    let new_path = functionality.step_in(forward_dirs.clone(), index);
    return (directory.find_forward_directories(), new_path.to_owned().to_str().unwrap().to_string());
}

// Building GUI and loading handlers.
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, process_response_step_in])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
