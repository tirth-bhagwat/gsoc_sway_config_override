mod config_parser;
mod config_reader;
mod enums;
mod utilities;

use config_parser::read_sway_config;
use config_reader::reader;
use std::env;
use std::process;

use crate::utilities::validate_path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Config file path required as an arguement...");
        process::exit(1);
    }
    let config_path = validate_path(&args[1], 2).unwrap_or_else(|err| {
        eprintln!("invalid file : {err}");
        process::exit(1);
    });

    let config = read_sway_config(&config_path).unwrap_or_else(|err| {
        eprintln!("Error while reading sway config at: {err}");
        process::exit(1);
    });

    let selected_files = reader(&config).unwrap_or_else(|err| {
        eprintln!("Error while reading : {err}");
        process::exit(1);
    });

    println!("Selected files : ");
    for f in selected_files {
        println!("{}", f.as_path().to_str().unwrap());
    }
}
