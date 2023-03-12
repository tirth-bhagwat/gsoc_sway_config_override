use crate::enums::Config;
use std::path::{Path, PathBuf};

#[allow(unused)]
pub fn reader<'a>(config: &'a Vec<Config>) -> Result<Vec<PathBuf>, &'static str> {
    let mut selected: Vec<PathBuf> = Vec::new();

    for conf in config {
        match conf {
            Config::Include { path } => {
                let files = Path::new(path)
                    .read_dir()
                    .expect("Cannot read direcrory contents")
                    .filter(|x| x.as_ref().unwrap().file_type().unwrap().is_file());

                for f in files {
                    let tmp = f.unwrap().path().as_path().to_owned();
                    if let Some(index) = selected
                        .iter()
                        .position(|x| x.file_name() == tmp.file_name())
                    {
                        selected[index] = tmp;
                    } else {
                        selected.push(tmp);
                    }
                }
            }
            Config::IncludeOne { paths } => {}
        }
    }

    Ok(selected)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reader() {
        let config = vec![
            Config::Include {
                path:
                    "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_1"
                        .to_string(),
            },
            Config::Include {
                path:
                    "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_2"
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

        let expected = vec![
            PathBuf::from("/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_2/a.conf"),PathBuf::from("/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_1/b.conf")
        ];

        assert_eq!(expected, reader(&config).unwrap());
    }
}
