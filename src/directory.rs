use walkdir::WalkDir;

#[allow(dead_code)]
pub struct Directory {
    pwd: String,
    forward_directories: Vec<String>,
    parent_directory: String,
}

impl Directory {
    pub fn new(pwd_param: String) -> Directory {
        Directory {
            pwd: pwd_param,
            forward_directories: Vec::new(),
            parent_directory: String::new(),
        }
    }

    pub fn new_empty() -> Directory {
        Directory {
            pwd: String::new(),
            forward_directories: Vec::new(),
            parent_directory: String::new(),
        }
    }

    pub fn find_forward_directories(&mut self) -> Vec<String> {
        let mut forward_directories: Vec<String> = Vec::new();
        for entry in WalkDir::new(&self.pwd).into_iter().filter_map(|e| e.ok()) {
            let not_split_file_path = entry.path().display().to_string();
            let split_file_path = not_split_file_path.split("\\");
            forward_directories.push(split_file_path.last().unwrap().to_string());
            self.set_forward_directories(forward_directories.clone());
        }
        return forward_directories;
    }

    pub fn find_parent_directory(&self) -> String {
        let split_file_path: Vec<&str> = self.pwd.split("\\").collect();
        if split_file_path.len() >= 2 {
            return split_file_path[split_file_path.len() - 2].to_string();
        } else {
            return String::new();
        }
    }

    pub fn set_pwd(&mut self, pwd: String) {
        self.pwd = pwd;
    }

    pub fn get_pwd(&self) -> String {
        self.pwd.clone()
    }

    pub fn set_forward_directories(&mut self, forward_directories: Vec<String>) {
        self.forward_directories = forward_directories;
    }

    pub fn get_forward_directories(&self) -> Vec<String> {
        self.forward_directories.clone()
    }

    fn set_parent_directory(&mut self, parent_directory: String) {
        self.parent_directory = parent_directory;
    }

    fn get_parent_directory(&self) -> String {
        self.parent_directory.clone()
    }
}