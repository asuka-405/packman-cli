use crate::utils;
use colored::*;

pub fn db() {
    let db: String = utils::get_db();
    let db = &db[1..db.len() - 1];
    crate::utils::fs::save_db(&db);
    const SUCCESS_MSG: &str = "Database refreshed successfully!";
    println!("{}", SUCCESS_MSG.green().bold().to_string());
}
