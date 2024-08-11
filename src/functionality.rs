use std::path::{Path, PathBuf};

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
}