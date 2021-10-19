use std::fs;
use std::error::Error;

pub struct Config {
    pub pattern: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Missing required argument!");
        }

        let pattern = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { pattern, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;
    println!("{}", file_content);
    Ok(())
}
