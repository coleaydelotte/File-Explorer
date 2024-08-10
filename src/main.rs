use walkdir::WalkDir;
fn main() {
    for entry in WalkDir::new("C:\\Users\\Colea\\Desktop\\CodingProjects\\Rust\\File-Explorer\\src").into_iter().filter_map(|e| e.ok()) {
        let mut file = entry.path().display().to_string();
        let files = file.split("\\");
        file = files.last().unwrap().to_string();
        
        println!("{}", file);
    }
}
