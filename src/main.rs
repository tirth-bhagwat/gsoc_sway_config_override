mod config_parser;
mod config_reader;
mod enums;
mod utilities;

use std::{path::PathBuf, process};

use config_parser::read_sway_config;
use config_reader::reader;
use shellexpand;

fn main() {
    let config_path = "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/config";

    let config = read_sway_config(config_path).unwrap_or_else(|err| {
        panic!("Error while reading sway config {err}");
    });

    let selected_files = reader(&config).unwrap_or_else(|err| {
        panic!("Error while reading sway config {err}");
    });

    ///////////////////////////////////////////////

    let x = "$HOME/files/conf/*";
    let y = PathBuf::from(x).canonicalize().unwrap_or_else(|err| {
        let p_str: String = shellexpand::full(x).unwrap().try_into().unwrap();
        println!("*(*(*(*(*(*(**(*(*(*(*(*((*(*");
        PathBuf::from(p_str)
    });
    // let p: String = shellexpand::full(x).unwrap().try_into().unwrap();

    dbg!(y);

    ///////////////////////////////////////////////

    dbg!(selected_files);
}
