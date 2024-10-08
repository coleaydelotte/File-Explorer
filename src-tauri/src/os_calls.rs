use std::process::Command;

/**
 * This function will open a file in the default program for the file type.
 * It will use the OS-specific command to open the file.
 */
pub fn open_file(file_path: &str) {
    // This function will open a file in the default program for the file type.
    // It will use the OS-specific command to open the file.
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", file_path])
            .output()
            .expect("Failed to open file.");
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(file_path)
            .output()
            .expect("Failed to open file.");
    }

    //linux is a maybe
    // #[cfg(target_os = "linux")]
    // {
    //     Command::new("xdg-open")
    //         .arg(file_path)
    //         .output()
    //         .expect("Failed to open file.");
    // }
}

pub fn get_os() -> String {
    let os;
    #[cfg(target_os = "macos")] {
        os = "macos".to_string();
    }
    #[cfg(target_os = "windows")] {
        os = "windows".to_string();
    }
    //linux is a maybe
    // #[cfg(target_os = "linux")] {
    //     os = "linux".to_string();
    // }
    os
}