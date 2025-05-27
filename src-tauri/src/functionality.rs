use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

pub struct Functionality {
    pwd : PathBuf,
    potential_steps : HashMap<i32,String>
}

/**
 * This struct is used to hold the current directory and provide
 * functionality to step up, step in, find the name of the current
 * directory, and output the files and directories in the current
 * directory to the console and to a file.
 */
impl Functionality {
    /**
     * This function creates a new Functionality struct with the given
     * path as the current directory.
     */
    pub fn new(pwd_param: PathBuf) -> Functionality {
        let func: Functionality = Functionality {
            pwd : pwd_param,
            potential_steps : HashMap::new()
        };
        func
    }

    /**
     * This function creates a new Functionality struct with an empty path.
     */
    pub fn new_empty() -> Functionality {
        let func: Functionality = Functionality {
            pwd : PathBuf::new(),
            potential_steps : HashMap::new()
        };
        func
    }

    /**
     * This function reads the pwd from the class and then returns
     * the file path of the parent directory.
     */
    pub fn step_up(&mut self) -> String {
        self.pwd = self.pwd.parent().unwrap().to_path_buf();
        println!("{}", self.pwd.display());
        return self.pwd.to_string_lossy().clone().to_string();
    }

    /**
     * This function reads the pwd from the class and then steps into
     * the first directory in the forward_directories vector.
     */
    pub fn step_in(&mut self, forward_directories: Vec<String>, index: i32) -> PathBuf {
        self.clear_potential_steps();
        let mut pwd_clone = self.pwd.clone();
        let mut input: i32;
    
        if forward_directories.is_empty() {
            return pwd_clone;
        }
    
        let mut iter = 1;
        for dir_name in forward_directories.iter() {
            let dir_path = Path::new(dir_name);
            let obj = WalkDir::new(dir_path).min_depth(1).max_depth(1);
            for entry in obj.into_iter().filter_map(|e| e.ok()) {
                self.add_to_potential_steps(iter, dir_name.clone());
                if entry.file_type().is_dir() {
                    iter += 1;
                }
            }
        }
    
        if index != 0 {
            input = index;
        } else {
            print!("Enter the number of the directory you would like to step into: ");
            let _ = stdout().flush();
        
            let mut input_string = String::new();
            stdin().read_line(&mut input_string).expect("Failed to read line");
            input = input_string.trim().parse().expect("Please type a number!");
        }
        input -= 1;
        pwd_clone.push(&forward_directories[input as usize]);
        self.pwd = pwd_clone.clone();
        pwd_clone
    }

    /**
     * This function reads the pwd from the class and then returns
     * the name of the current directory without the rest of the path.
     */
    pub fn find_pwd_name(&mut self) -> String {
        let pwd_name = self.pwd.file_name().map(|name| name.to_string_lossy().into_owned());
        match pwd_name {
            Some(name) => name,
            None => String::new(),
        }
    }

    /**
     * Takes a vector of strings as parameter containing the names of the forward files,
     * and returns a vector of u64 integers representing the sizes of the files.
     */
    pub fn find_file_sizes(&mut self, forward_files: Vec<String>) -> Vec<u64> {
        let mut file_sizes = Vec::new();
        for file_name in forward_files {
            let file_path = self.pwd.join(file_name);
            if let Ok(metadata) = file_path.metadata() {
                file_sizes.push(metadata.len());
            }
        }
        file_sizes
    }

    /**
     * This function clears the potential_steps HashMap.
     */
    fn clear_potential_steps(&mut self) {
        self.potential_steps.clear();
    }

    /**
     * This function takes a i32 and a String and adds a new index to the potential_steps HashMap.
     */
    fn add_to_potential_steps(&mut self, iter: i32, step: String) {
        self.potential_steps.insert(iter, step);
    }

    /**
     * WARNING: When outputting files to the `output.txt` file it will wipe it clean 
     * then write the new files and directories to it.
     * 
     * This function reads the pwd from the class and then outputs
     * the files and directories in the current directory to the output file, and to the console.
     */
    pub fn output_files(&mut self, forward_dirs: Vec<String>, print_files: bool) {
        let mut dir_iter: i32 = 1;
        let mut iter: i32 = 1;
        let pwd_clone = self.pwd.clone();

        for dir_name in forward_dirs {
            let dir_path = PathBuf::from(pwd_clone.join(dir_name));

            if dir_path.is_dir() {
                println!("Directory {}: {}", dir_iter, dir_path.iter().last().unwrap().to_string_lossy());
                dir_iter += 1;

            }
            if print_files && dir_path.is_file() {
                let file_name = dir_path.file_name().unwrap().to_string_lossy();
                println!("File {}: {}", iter, file_name);
                iter += 1;
            }
        }
    }

    /**
     * This function reads the pwd from the class and then outputs
     * the files and directories in the current directory to the output file.
     */
    pub fn output_files_to_file(&mut self, forward_dirs: Vec<String>, print_files: bool, output_name: String) {
        let mut dir_iter: i32 = 1;
        let mut iter: i32 = 1;
        let pwd_clone = self.pwd.clone();

        let mut output = String::new();
        for dir_name in forward_dirs {
            let dir_path = PathBuf::from(pwd_clone.join(dir_name));

            if dir_path.is_dir() {
                output.push_str(&format!("Directory {}: {}\n", dir_iter, dir_path.iter().last().unwrap().to_string_lossy()));
                dir_iter += 1;

            }
            if print_files && dir_path.is_file() {
                let file_name = dir_path.file_name().unwrap().to_string_lossy();
                output.push_str(&format!("File {}: {}\n", iter, file_name));
                iter += 1;
            }
        }
        std::fs::write(output_name, output).expect("Unable to write file");
    }

    /**
     * This function reads the pwd from the class and then returns what would be printed
     * as a vector of strings to be used in the Tauri application.
     */
    pub fn output_files_as_vector(&mut self, forward_dirs: Vec<String>, print_files: bool) -> Vec<String> {
        let mut dir_iter: i32 = 1;
        let mut iter: i32 = 1;
        let pwd_clone = self.pwd.clone();
        let mut output: Vec<String> = Vec::new();

        for dir_name in forward_dirs {
            let dir_path = PathBuf::from(pwd_clone.join(dir_name));

            if dir_path.is_dir() {
                output.push(format!("Directory {}: {}", dir_iter, dir_path.iter().last().unwrap().to_string_lossy()));
                dir_iter += 1;

            }
            if print_files && dir_path.is_file() {
                let file_name = dir_path.file_name().unwrap().to_string_lossy();
                output.push(format!("File {}: {}", iter, file_name));
                iter += 1;
            }
        }
        return output;
    }

    /**
     * This function returns the current working directory.
     */
    pub fn get_pwd(&self) -> PathBuf {
        return  self.pwd.clone();
    }

    /**
     * This function formats the path passed as parameter for Windows.
     */
    pub fn format_path_for_windows(&self, mut path: String) -> String {
        path = path.replace("\\", "\\\\");
        return path;
    }

    /**
     * Clears the terminal screen based on the operating system.
     */
    pub fn clear_terminal(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg("cls")
                .status()
                .expect("Failed to clear terminal");
        }
        if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
            Command::new("clear")
                .status()
                .expect("Failed to clear terminal");
        }
    }

    /**
     * Gets file size for a file and returns it.
     */
    pub fn get_size(&self, file_path: PathBuf) -> u64 {
        let metadata = file_path.metadata().unwrap();
        metadata.len()
    }
}