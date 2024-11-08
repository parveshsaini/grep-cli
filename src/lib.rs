use std::{ error::Error, fs::read_to_string};


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents= read_to_string(&config.filename)?;

    println!("contents of file are: {}", contents);

    Ok(())

}

pub struct Config{
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len()<3 {
            return Err("Very few arguements!");
        }

        let query= args[1].clone();
        let filename= args[2].clone();

        return Ok(Config { query, filename});
    }
}