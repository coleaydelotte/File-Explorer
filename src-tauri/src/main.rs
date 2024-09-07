// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_loop;
mod functionality;
mod directory;
mod os_calls;
mod front_end_util;

// Building GUI and loading Utility.
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            front_end_util::greet, 
            front_end_util::step_in, 
            front_end_util::step_up, 
            front_end_util::open_file, 
            front_end_util::output_files_as_vector, 
            front_end_util::get_os,
            front_end_util::format_path_for_windows
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
