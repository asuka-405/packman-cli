pub mod refresh;
pub mod sync;
pub mod update;

const VERSION: &'static str = "0.1.0-α";

pub fn get_version() -> &'static str {
    VERSION
}

pub fn print_version() {
    println!("PackMan Version: {}", VERSION);
}
