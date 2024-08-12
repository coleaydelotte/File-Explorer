use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Directory {
    pwd: PathBuf,
    forward_directories: Vec<String>,
    parent_directory: String,
}

impl Directory {
    pub fn new(pwd_param: PathBuf) -> Directory {
        let mut directory = Directory {
            pwd: pwd_param,
            forward_directories: Vec::new(),
            parent_directory: String::new(),
        };
        directory.update_values();
        directory
    }

    pub fn new_empty() -> Directory {
        Directory {
            pwd: PathBuf::new(),
            forward_directories: Vec::new(),
            parent_directory: String::new(),
        }
    }

    pub fn find_forward_directories(&mut self) -> Vec<String> {
        let mut forward_directories = Vec::new();
        for entry in WalkDir::new(&self.pwd).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            if let Some(file_name) = entry.path().file_name() {
                forward_directories.push(file_name.to_string_lossy().into_owned());
            }
        }
        self.forward_directories = forward_directories.clone();
        forward_directories
    }

    pub fn find_parent_directory(&mut self) -> String {
        if let Some(parent) = self.pwd.parent().and_then(|p| p.file_name()) {
            let parent_directory = parent.to_string_lossy().into_owned();
            self.parent_directory = parent_directory.clone();
            parent_directory
        } else {
            String::new()
        }
    }

    fn update_values(&mut self) {
        self.find_forward_directories();
        self.find_parent_directory();
    }

    pub fn set_pwd(&mut self, pwd: PathBuf) {
        self.pwd = pwd;
        self.update_values();
    }

    pub fn get_pwd(&self) -> PathBuf {
        self.pwd.clone()
    }

    pub fn get_forward_directories(&self) -> Vec<String> {
        self.forward_directories.clone()
    }

    pub fn get_parent_directory(&self) -> String {
        self.parent_directory.clone()
    }
}
