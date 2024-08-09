use crate::utils::{db, fs};
use std::{
    env,
    path::PathBuf,
    process::{self, Command},
};

pub fn install(package: &str) {
    let fallback: String = format!("{}/packman/", fs::get_homedir());
    let curdir: PathBuf = match env::current_dir() {
        Ok(cwd) => cwd,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
    let curdir: String = match curdir.to_str() {
        Some(cwd) => cwd.to_string(),
        None => fallback,
    };
    let curdir: String = format!("{}/packman/", curdir);

    let db: String = format!("{}/packman/", fs::get_homedir());
    let db: String = format!("{}/core.db", db);
    let db: String = fs::get_file(&db);
    let db: db::Database = db::Database::parse(&db);

    let package = match db.find_package(package) {
        Some(p) => p,
        None => {
            eprintln!("Error: Package not found");
            process::exit(1);
        }
    };

    let install_path: String = format!("{}/{}", curdir, package.name);

    match Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg(&package.repo)
        .arg(&install_path)
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "{} clone failed with status: {}",
                    package.name, output.status
                );
                if !output.stderr.is_empty() {
                    eprintln!("Error output: {}", String::from_utf8_lossy(&output.stderr));
                }
                process::exit(1);
            } else {
                println!("{} clone completed successfully", package.name);
                if !output.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
