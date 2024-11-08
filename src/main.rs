use std::{env, process};
use grepcli::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config= Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("searching {} in file {} ...", config.query, config.filename);

    if let Err(err) = grepcli::run(config) {
        println!("error in run func: {}", err);
        process::exit(1);
    }

}
