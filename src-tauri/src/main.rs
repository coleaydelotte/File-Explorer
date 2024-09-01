// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_loop;
mod functionality;
mod directory;
mod os_calls;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn main_loop() {
//     main_loop::main_loop();
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        // .invoke_handler(tauri::generate_handler![main_loop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
