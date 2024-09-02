// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
fn step_in(response: String, pwd: String) -> (Vec<String>, String) {
    let forward_dirs;
    let new_path;
    (forward_dirs, new_path) = main_loop::process_response_step_in(&response, pwd);
    return (forward_dirs, new_path.to_string());
}

#[tauri::command]
fn step_up(path: String) -> (Vec<String>, String) {
    let forward_dirs;
    let new_path;
    (forward_dirs, new_path) = main_loop::process_response_step_up(&path);
    return (forward_dirs, new_path.to_string());
}

fn open() {

}

// Building GUI and loading Utility.
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, step_in, step_up])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
