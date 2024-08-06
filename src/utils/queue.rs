use super::usage;
use crate::tasks;

pub fn setup_queue(queue: &mut Vec<String>, cli_args: Vec<String>) {
    for c in cli_args[1].chars() {
        match c {
            '-' => (),
            'h' => usage::print_usage(),
            'v' => tasks::print_version(),

            'S' => {
                let s: String = ["S;;", cli_args[2].as_str()].concat();
                queue.push(s);
            }
            'y' => queue.push("y".to_string()),
            'u' => queue.push("u".to_string()),
            _ => {
                usage::print_usage();
                std::process::exit(1);
            }
        };
    }
    reorder_queue(queue)
}

fn reorder_queue(queue: &mut Vec<String>) {
    let mut indices_to_reorder: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < queue.len() {
        if queue[i].as_str().contains("S") {
            indices_to_reorder.push(i as u8);
        }
        i += 1;
    }
    for &index in indices_to_reorder.iter().rev() {
        let task = queue.remove(index as usize);
        queue.push(task);
    }
}

pub fn execute_queue(queue: Vec<String>) {
    for task in queue {
        match task.as_str() {
            "y" => tasks::refresh::db(),
            "u" => tasks::update::packages(),
            _ => {
                tasks::sync::install(task.split(";;").collect::<Vec<&str>>()[1]);
            }
        }
    }
}
