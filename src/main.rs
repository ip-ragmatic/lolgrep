use lolgrep::{run, Config};
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("lolgrep: {err}");
        process::exit(1)
    });
    if let Err(e) = run(&config) {
        eprintln!("lolgrep: {}: {}", &config.path, e);
        process::exit(1)
    }
}
