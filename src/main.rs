use std::env;
use std::process;

use tret_cli::Config;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("Problem while parsing arguments: {}", err);
        process::exit(1);
    });

    let response  = tret_cli::request(&config.query);

}
