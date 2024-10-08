pub mod db;
pub mod fs;
pub mod usage;

use reqwest::blocking::get;
use std::process;

const DB_STORE_URL: &str = "https://packman.ksuryansh.xyz/.netlify/functions/coredb";

pub fn get_db() -> String {
    match get(DB_STORE_URL) {
        Ok(res) => match res.text() {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
