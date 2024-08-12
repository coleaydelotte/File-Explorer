use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Write;
use crate::directory;

pub struct Functionality {
    pwd : PathBuf,
    forward_directories : Vec<String>,
    parent_directory : String,
}

impl Functionality {
    pub fn new(pwd_param: PathBuf, forward_directories_param: Vec<String>) -> Functionality {
        let mut func: Functionality = Functionality {
            pwd : pwd_param,
            forward_directories : forward_directories_param,
            parent_directory : String::new(),
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
        self.parent_directory = pwd_clone.to_string_lossy().into_owned();
        return pwd_clone.to_string_lossy().into_owned();
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
        for entry in WalkDir::new(directory.get_pwd()).max_depth(1).into_iter() {
            match entry {
                Ok(path) => {
                    if path.path() == directory.get_pwd() {
                        continue;
                    }
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