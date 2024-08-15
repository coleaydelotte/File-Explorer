use std::path::PathBuf;
mod directory;
mod functionality;

fn main() {
    println!("Welcome to the File Explorer!");

    let mut directory = directory::Directory::new_empty();
    let target_path = PathBuf::from("C:\\Users\\Colea\\Desktop");
    directory.set_pwd(target_path.clone());
    println!("Scanning directory: {}", directory.get_pwd().display());
    let mut functionality = functionality::Functionality::new(directory.get_pwd());
    println!("Stepping Up A Directory: {}", functionality.step_up());
    directory.set_pwd(PathBuf::from(functionality.get_pwd()));
    let forward_dirs: Vec<String> = directory.find_forward_directories();
    println!("{:?}", forward_dirs);
    functionality.output_files(forward_dirs.clone(), false);
    let new_path: PathBuf = functionality.step_in(forward_dirs);
    directory.set_pwd(new_path.clone());
    println!("Stepping In A Directory: {}", new_path.display());
    functionality.output_files(directory.find_forward_directories_and_files(), true);
}

