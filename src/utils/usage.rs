use colored::*;

pub fn print_usage() {
    let mut usage: String = " Usage: packman [flags] [arg] "
        .bold()
        .on_bright_black()
        .to_string();
    usage.push_str("\nFlags:\n".yellow().to_string().as_str());
    usage.push_str("  -h\n".yellow().to_string().as_str());
    usage.push_str("        show usage.\n".magenta().to_string().as_str());
    usage.push_str("  -v\n".yellow().to_string().as_str());
    usage.push_str(
        "        show packman version and exit.\n"
            .magenta()
            .to_string()
            .as_str(),
    );
    usage.push_str("  -S\n".yellow().to_string().as_str());
    usage.push_str(
        "        pull code from publisher. all remote code will be replaced with publisher code.\n"
            .magenta()
            .to_string()
            .as_str(),
    );
    usage.push_str("  -y\n".yellow().to_string().as_str());
    usage.push_str("        refresh coredb.\n".magenta().to_string().as_str());
    usage.push_str("  -u\n".yellow().to_string().as_str());
    usage.push_str(
        "        update all packages. all edited code will be replaced with publisher code.\n"
            .magenta()
            .to_string()
            .as_str(),
    );
    println!("{}", usage);
}
