use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip executable name

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("No pattern given!"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No file given!"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { pattern, filename, case_sensitive })
    }
}

pub fn search<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let lower_pattern = pattern.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&lower_pattern))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.pattern, &file_content)
    } else {
        search_case_insensitive(&config.pattern, &file_content)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}


#[cfg(test)]
mod tests {
   use super::*; 

    #[test]
    fn case_sensitive() {
        let keyword = "duck";
        let content = "\
Rust is statically typed.
Not duck typed like Python.
If it looks like that, it's probably one.
        ";
        assert_eq!(vec!["Not duck typed like Python."], search(keyword, content));
    }

    #[test]
    fn case_insensitive() {
        let keyword = "dasH";
        let content = "\
They are a lot of shapes on the dashboard.
We can say, for instance:
- dashed lines
- squares, etc...";
        assert_eq!(
            vec!["They are a lot of shapes on the dashboard.", "- dashed lines"],
            search_case_insensitive(keyword, content)
        );
    }
}
