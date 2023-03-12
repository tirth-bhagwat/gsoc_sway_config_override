mod config_parser;
mod enums;
mod utilities;
mod config_reader;

use std::fs;
use std::path::Path;
use std::process;


use config_parser::read_sway_config;

fn main() {
    let config_path = "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/config";

    let config = read_sway_config(config_path).unwrap_or_else(|err| {
        eprintln!("Error while reading sway config {err}");
        process::exit(1);
    });

    dbg!(config);

    let p = "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_1";
    let x = fs::read_dir(p).unwrap();
    for i in x {
        println!("{:?}", i.unwrap().path());
    }
}
