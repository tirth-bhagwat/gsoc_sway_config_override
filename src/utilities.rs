use std::path::Path;

/// Takes 3 codes:
/// - 0 => checks if path is valid
/// - 1 => checks if path is valid directory
/// - 2 => checks if path is valid file
pub fn validate_path<'a>(path: &'a str, code: i8) -> Result<String, String> {
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

pub fn format_path(path: String) -> String {
    let mut result = String::from(path.trim());
    if result.ends_with("/*") {
        result.pop().unwrap();
        result.pop().unwrap();
    } else if result.ends_with("/") {
        result.pop().unwrap();
    }
    result
}
