use std::{env, process};

use book_study_unit3::Config;

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", &config.query);
    println!("Searching path {}", &config.file_path);

    println!("{:?}", config);


    if let Result::Err(err) = book_study_unit3::run(config) {
        eprintln!("Application error! {err}");
        process::exit(1);
    }
}
