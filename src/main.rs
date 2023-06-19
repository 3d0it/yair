mod configuration;
mod resizer;
use crate::configuration::Config;
use clap::Parser;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = resizer::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
