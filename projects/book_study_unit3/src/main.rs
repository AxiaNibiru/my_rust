use std::{env, process};

use book_study_unit3::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", &config.query);
    println!("Searching path {}", &config.file_path);

    if let Result::Err(err) = book_study_unit3::run(config) {
        println!("Application error! {err}");
        process::exit(1);
    }
}
