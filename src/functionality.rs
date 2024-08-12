use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::fs::File;
use std::io::Write;
use crate::directory;

pub struct Functionality {
    pwd : PathBuf,
}

impl Functionality {
    pub fn new(pwd_param: PathBuf) -> Functionality {
        let mut func: Functionality = Functionality {
            pwd : pwd_param,
        };
        func.step_up();
        func
    }

    /**
     * This function reads the pwd from the class and then returns
     * the file path of the parent directory.
     */
    pub fn step_up(&mut self) -> String {
        let mut pwd_clone = self.pwd.clone();
        pwd_clone.pop();
        return pwd_clone.to_string_lossy().into_owned();
    }

    pub fn step_in(&mut self, forward_directories: Vec<String>) -> PathBuf {
        let mut pwd_clone = self.pwd.clone();
    
        if forward_directories.is_empty() {
            return pwd_clone;
        }
    
        let mut iter = 1;
        for dir_name in forward_directories.iter() {
            println!("Directory {}: {}", iter, dir_name);
            let dir_path = Path::new(dir_name);
            let obj = WalkDir::new(dir_path).min_depth(1).max_depth(1); // Use max_depth(1) to limit to immediate contents
            for entry in obj.into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_dir() {
                    iter += 1;
                }
            }
        }
        pwd_clone.push(&forward_directories[0]); // Assuming you want to step into the first directory
        pwd_clone
    }

    pub fn find_pwd_name(&mut self) -> String {
        let pwd_name = self.pwd.file_name().map(|name| name.to_string_lossy().into_owned());
        match pwd_name {
            Some(name) => name,
            None => String::new(),
        }
    }

    pub fn output_files(&mut self, directory: &mut directory::Directory, output_file_path: &str) {
        let mut iter: i32 = 1;
        let mut dir_iter: i32 = 1;
        let mut output_file = File::create(output_file_path).expect("Could not create file");
        for entry in WalkDir::new(directory.get_pwd()).min_depth(1).max_depth(1).into_iter() {
            match entry {
                Ok(path) => {
                    if path.path().is_file() {
                        let file_name = path.path().file_name().unwrap().to_string_lossy();
                        writeln!(output_file, "File {}: {}", iter, file_name).expect("Could not write to file");
                        println!("File {}: {}", iter, file_name);
                        iter += 1;
                    } else if path.path().is_dir() {
                        let dir_name = path.path().file_name().unwrap().to_string_lossy();
                        writeln!(output_file, "Directory {}: {}", dir_iter, dir_name).expect("Could not write to file");
                        println!("Directory {}: {}", dir_iter, dir_name);
                        dir_iter += 1;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading entry: {}", e);
                }
            }
        }
    }
}