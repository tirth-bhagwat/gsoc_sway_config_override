use std::fs;
use std::path::Path;

const KW_INCLUDE: &str = "include";
const KW_INCLUDE_ONE: &str = "include_one";
#[derive(Debug, PartialEq)]
pub enum Config {
    Include { path: String },
    IncludeOne { paths: Vec<String> },
}

/// Takes 3 codes:
/// - 0 => checks if path is valid
/// - 1 => checks if path is valid directory
/// - 2 => checks if path is valid file
fn validate_path<'a>(path: &'a str, code: i8) -> Result<String, String> {
    let result = format_path(String::from(path));
    let is_valid = match code {
        0 => Path::new(result.as_str()).exists(),
        1 => Path::new(result.as_str()).exists() && Path::new(result.as_str()).is_dir(),
        2 => Path::new(result.as_str()).exists() && Path::new(result.as_str()).is_file(),
        _ => false,
    };
    if is_valid == false {
        return Err(result.to_string());
    } else {
        return Ok(result.to_string());
    }
}

fn format_path(path: String) -> String {
    let mut result = String::from(path.trim());
    if result.ends_with("/*") {
        result.pop().unwrap();
        result.pop().unwrap();
    } else if result.ends_with("/") {
        result.pop().unwrap();
    }
    result
}

pub fn read_sway_config(path: &str) -> Result<Vec<Config>, &'static str> {
    let config_text = fs::read_to_string(&path).expect("Unable to read file ");

    let mut configs = Vec::new();

    for line in config_text.lines() {
        if line.starts_with(KW_INCLUDE_ONE) {
            let mut x = line.split(" ");

            x.next().expect("Error reading config file");
            let mut paths = Vec::new();

            for path in x {
                let p = validate_path(path.trim(), 1).expect("Not a valid directory ");
                paths.push(format_path(p));
            }
            configs.push(Config::IncludeOne { paths })
        } else if line.starts_with(KW_INCLUDE) {
            let mut x = line.split(" ");

            x.next().expect("Error reading config file");

            let path = x.next().expect("Error reading config file ").to_string();
            let path = validate_path(&path, 1).expect("Invalid directory ");

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

        dbg!(&expexted);
        dbg!(&read_sway_config(&path).unwrap());

        assert_eq!(expexted, read_sway_config(&path).unwrap());
    }
}
