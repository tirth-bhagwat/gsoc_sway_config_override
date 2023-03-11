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
/// - _ => returns false
fn validate_path<'a>(file: &'a str, code: i8) -> Result<&'a str, &'a str> {
    let result = match code {
        0 => Path::new(file).exists(),
        1 => Path::new(file).exists() && Path::new(file).is_dir(),
        2 => Path::new(file).exists() && Path::new(file).is_file(),
        _ => false,
    };
    if result == false {
        return Err(file);
    } else {
        return Ok(file);
    }
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
                paths.push(p.to_string());
            }
            configs.push(Config::IncludeOne { paths })
        } else if line.starts_with(KW_INCLUDE) {
            let mut x = line.split(" ");

            x.next().expect("Error reading config file");

            let path = x.next().expect("Error reading config file ").to_string();
            let path = validate_path(&path, 1).expect("Invalid directory ");

            configs.push(Config::Include {
                path: path.to_string(),
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
                path: "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_1/".to_string(),
            },
            Config::IncludeOne {
                paths: vec![
                    "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_1/"
                        .to_string(),
                    "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_2/".to_string(),
                ],
            },
        ];
        let path = "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/config";

        dbg!(&expexted);
        dbg!(&read_sway_config(&path).unwrap());

        assert_eq!(expexted, read_sway_config(&path).unwrap());
    }
}
