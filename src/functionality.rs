pub struct Functionality {
    pwd : String,
    forward_directories : Vec<String>,
    parent_directory : String,
}

impl Functionality {
    pub fn new(pwd_param: String, forward_directories_param: Vec<String>) -> Functionality {
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
        let pwd_clone = self.pwd.clone();
        let mut split_file_path: Vec<&str> = pwd_clone.split("\\").collect();
        split_file_path.pop();
        split_file_path.pop();
        let mut parent_directory: String = String::new();
        for i in split_file_path {
            parent_directory.push_str(i);
            parent_directory.push_str("\\");
        }
        parent_directory
    }
}