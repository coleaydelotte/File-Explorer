use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
mod directory;
mod functionality;

fn main() {
    println!("Welcome to the File Explorer!");

    let output_path: &str = "output.txt";
    let mut output_file = File::create(output_path).expect("Could not create file");
    let mut directory = directory::Directory::new_empty();
    let target_path = PathBuf::from("C:\\Users\\Colea\\Desktop\\CodingProjects");
    directory.set_pwd(target_path.clone());
    println!("Scanning directory: {}", directory.get_pwd().display());
    let mut forward_dirs: Vec<String> = directory.find_forward_directories_and_files();
    let mut functionality = functionality::Functionality::new(directory.get_pwd());
    println!("Stepping Up A Directory: {}", functionality.step_up());
    directory.set_pwd(PathBuf::from(functionality.get_pwd()));
    functionality.output_files(&mut directory, output_path, false);
    forward_dirs = directory.find_forward_directories();
    let new_path: PathBuf = functionality.step_in(forward_dirs);
    directory.set_pwd(new_path.clone());
    println!("Stepping In A Directory: {}", new_path.display());
    writeln!(output_file, "Stepping In A Directory: {}", new_path.display()).expect("Could not write to file");
    functionality.output_files(&mut directory, output_path, true);
}
