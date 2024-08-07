use crate::utils;

pub fn db() {
    let db: String = utils::get_db();
    crate::utils::fs::save_db(&db);
    println!("Database refreshed successfully!");
}
