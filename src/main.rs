use std::{env, process};
use grepcli::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config= Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("🔍 searching {} in file {} ...", config.query, config.filename);

    if let Err(err) = grepcli::run(config) {
        eprintln!("error in run func: {}", err);
        process::exit(1);
    }

}
