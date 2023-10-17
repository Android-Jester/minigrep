use minigrep::{run, Config};
use std::{env, process};


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing Arguments: {err}");
        process::exit(1);
    });

    println!(
        "[+] Searching for \"{}\", in file {}, ignorecase: {}",
        config.query, config.file_path, config.ignore_case
    );
    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
