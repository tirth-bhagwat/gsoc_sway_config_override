use crate::enums::Config;
use crate::utilities::*;
use std::{error::Error as ErrorTrait, fs};

const KW_INCLUDE: &str = "include";
const KW_INCLUDE_ONE: &str = "include_one";

// pub fn read_sway_config(path: &str) -> Result<Vec<Config>, String> {
pub fn read_sway_config(path: &str) -> Result<Vec<Config>, Box<dyn ErrorTrait>> {
    let cfg = validate_path(&path, 2)?;
    let config_text = fs::read_to_string(&cfg)?;

    let mut configs = Vec::new();

    for line in config_text.lines() {
        if line.starts_with(KW_INCLUDE_ONE) {
            let mut x = line.split(" ");

            x.next().expect("Error reading config file...");
            let mut paths = Vec::new();

            for path in x {
                let p = validate_path(path.trim(), 1)?;
                paths.push(format_path(p));
            }
            configs.push(Config::IncludeOne { paths })
        } else if line.starts_with(KW_INCLUDE) {
            let mut x = line.split(" ");

            x.next().expect("Error reading config file...");

            let path = x.next().expect("Error reading config file...").to_string();
            let path = validate_path(&path, 1)?;

            configs.push(Config::Include {
                path: format_path(path),
            });
        }
    }

    Ok(configs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_sway_config() {
        let expexted = vec![
            Config::Include {
                path:
                    "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_1"
                        .to_string(),
            },
            Config::IncludeOne {
                paths: vec![
                    "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_1"
                        .to_string(),
                    "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_2"
                        .to_string(),
                ],
            },
        ];
        let path = "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/config";

        assert_eq!(expexted, read_sway_config(&path).unwrap());
    }
}
