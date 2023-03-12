use std::path::PathBuf;

/// Takes 3 codes:
/// - 0 => checks if path is valid
/// - 1 => checks if path is valid directory
/// - 2 => checks if path is valid file
pub fn validate_path<'a>(path: &'a str, code: i8) -> Result<String, String> {
    let result = format_path(String::from(path));

    let result = PathBuf::from(&result).canonicalize().unwrap_or_else(|_| {
        let p_str: String = shellexpand::full(&result).unwrap().try_into().unwrap();
        PathBuf::from(p_str)
    });
    let is_valid = match code {
        0 => result.exists(),
        1 => result.exists() && result.is_dir(),
        2 => result.exists() && result.is_file(),
        _ => false,
    };
    if is_valid == false {
        return Err(result.as_path().to_str().unwrap().to_string());
    } else {
        return Ok(result.as_path().to_str().unwrap().to_string());
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
