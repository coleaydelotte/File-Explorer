use std::path::PathBuf;
use walkdir::WalkDir;
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

    let forward_dirs: Vec<String> = directory.find_forward_directories();
    let mut functionality = functionality::Functionality::new(directory.get_pwd(), forward_dirs.clone());


    functionality.output_files(&mut directory, output_path);
    println!();
    writeln!(output_file, "Parent Directory: {}", directory.find_parent_directory()).expect("Could not write to file");
    println!("Parent Directory: {}", directory.find_parent_directory());
    println!();
    writeln!(output_file, "Current Directory: {}", directory.get_pwd().display()).expect("Could not write to file");
    println!("Current Directory: {}", directory.get_pwd().display());
    println!();
    println!("Stepping Up A Directory: {}", functionality.step_up());
    writeln!(output_file, "Stepping Up A Directory: {}", functionality.step_up()).expect("Could not write to file");
    directory.set_pwd(PathBuf::from(functionality.step_up()));
    functionality.output_files(&mut directory, output_path);
}
