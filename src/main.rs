// use walkdir::WalkDir;
mod directory;
mod functionality;

fn main() {
    // for entry in WalkDir::new("C:\\Users\\Colea\\Desktop\\CodingProjects\\Rust\\File-Explorer\\src").into_iter().filter_map(|e| e.ok()) {
    //     let mut file = entry.path().display().to_string();
    //     let files = file.split("\\");
    //     file = files.last().unwrap().to_string();
    //     println!("{}", file);
    // }
    let mut directory = directory::Directory::new_empty();
    directory.set_pwd("C:\\Users\\Colea\\Desktop\\CodingProjects\\Rust\\File-Explorer\\src\\".to_string());
    let forward_dirs: Vec<String> = directory.find_forward_directories();
    let mut iter: i32 = 1;
    let mut functionality = functionality::Functionality::new(directory.get_pwd(), forward_dirs.clone());
    for i in forward_dirs {
        if i == "" {
            continue;
        }
        else {
            println!("File {}: {}", iter, i);
            iter += 1;
        }
    }
    println!();
    print!("Parent Directory: ");
    println!("{}", directory.find_parent_directory());
    println!();
    println!("Current Directory: {}", directory.get_pwd());
    println!();
    println!("Parent Directory Full: {}", functionality.step_up());
}
