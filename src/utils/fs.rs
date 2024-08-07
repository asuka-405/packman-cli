use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process::{self, Command},
};

const DB_STORE: (&str, &str) = ("~/.packman/core.db", "C:\\Program Files\\Packman\\core.db");

fn create_or_overwrite_file(file_path: &str, content: &str) {
    if let Some(parent) = Path::new(file_path).parent() {
        match create_dir_all(parent) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
    match file.write_all(content.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

pub fn save_db(content: &str) {
    if cfg!(target_os = "windows") {
        eprintln!("Windows is not supported yet!");
        process::exit(1);
    } else if cfg!(target_os = "macos") {
        eprintln!("macOS is not supported yet!");
        process::exit(1);
    } else if cfg!(target_os = "linux") {
        let homedir = match Command::new("sh").arg("-c").arg("echo $HOME").output() {
            Ok(output) => String::from_utf8(output.stdout).unwrap(),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        };
        let homedir = format!("{}/packman/core.db", homedir.trim());
        create_or_overwrite_file(&homedir, content)
    }
}
