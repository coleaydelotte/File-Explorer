use std::path::PathBuf;
use walkdir::WalkDir;
mod directory;
mod functionality;

fn main() {
    let mut directory = directory::Directory::new_empty();
    directory.set_pwd(PathBuf::from("C:\\Users\\Colea\\Desktop\\CodingProjects\\Rust\\File-Explorer\\src"));
    let forward_dirs: Vec<String> = directory.find_forward_directories();
    let mut functionality = functionality::Functionality::new(directory.get_pwd(), forward_dirs.clone());
    let mut iter: i32 = 1;
    let mut dir_iter: i32 = 1;
    for entry in WalkDir::new(directory.get_pwd()).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path == directory.get_pwd() {
            continue;
        }
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy();
            println!("File {}: {}", iter, file_name);
            iter += 1;
        } else if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_string_lossy();
            println!("Directory {}: {}", dir_iter, dir_name);
            dir_iter += 1;
        }
    }
    println!();
    print!("Parent Directory: ");
    println!("{}", directory.find_parent_directory());
    println!();
    println!("Current Directory: {:?}", directory.get_pwd());
    println!();
    println!("Stepping Up A Directory: {}", functionality.step_up());
}
