use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(q) => q,
            None => return Err("lolgrep: missing query"),
        };
        let path = match args.next() {
            Some(p) => p,
            None => return Err("lolgrep: missing file path to search in"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok_and(|f| f == "1");
        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_data = fs::read_to_string(&config.path)?;
    let res = if config.ignore_case {
        search_case_insensitive(&config.query, &file_data)
    } else {
        search(&config.query, &file_data)
    };
    res.iter()
        .for_each(|(i, line)| println!("{}:{line}", i + 1));

    Ok(())
}

pub fn search<'a>(query: &str, data: &'a str) -> Vec<(usize, &'a str)> {
    data.lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, data: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    data.lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .collect()
}
