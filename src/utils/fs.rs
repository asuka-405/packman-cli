use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::Path,
    process::{self, Command},
};

const DB_STORE: (&str, &str) = ("~/.packman/core.db", "C:\\Program Files\\Packman\\core.db");

pub fn get_file(file_path: &str) -> String {
    let mut file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

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
        let homedir = get_homedir();
        let homedir = format!("{}/packman/core.db", homedir.trim());
        create_or_overwrite_file(&homedir, content)
    }
}

pub fn get_homedir() -> String {
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
        format!("{}/packman", homedir.trim())
    } else {
        eprintln!("Unsupported OS!");
        process::exit(1);
    }
}

pub fn git_clone(url: &str, path: &str) {
    let status = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg(url)
        .arg(path)
        .status()
        .expect("Failed to clone repository");
    if !status.success() {
        eprintln!("Failed to clone repository");
        process::exit(1);
    }
}
