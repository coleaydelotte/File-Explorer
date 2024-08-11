// use walkdir::WalkDir;
mod directory;

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
}
