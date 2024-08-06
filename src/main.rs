mod tasks;
mod utils;

use std::{self, env};
use utils::{queue, usage};

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() != 3 {
        usage::print_usage();
        std::process::exit(1);
    }

    let mut queue: Vec<String> = Vec::new();
    queue::setup_queue(&mut queue, cli_args);
    queue::execute_queue(queue);
}
