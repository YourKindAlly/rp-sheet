use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;
use std::path::PathBuf;
/**
 * @license MIT License
 * @author Jasmine Regnér
 */

/// Creates the config file in "~/.config/rp-sheet/config.json" if one doesn't already exist.
pub fn create_config_file(config_contents: &ConfigContents) -> Result<()> {
    process_directory();

    let path = PathBuf::from("~/.config/rp-sheet/config.json");

    if is_existing_path(&path) {
        return Ok(());
    }

    let json = serde_json::to_string(config_contents).unwrap();
    fs::write(path, json)
}

/// Ensures that the directory exists or is created. Panics if there is an error in the process.
fn process_directory() {
    match create_config_directory() {
        Ok(_result) => {}
        Err(err) => panic!("There was an error when processing the config file: {err:?}"),
    }
}

/// Creates the directory in which the config file will be created into if it doesn't already exist.
fn create_config_directory() -> Result<()> {
    let path = PathBuf::from("~/.config/rp-sheet");

    if is_existing_path(&path) {
        return Ok(());
    }

    fs::create_dir_all(&path)?;
    Ok(())
}

/// Returns true if the path exists, and false if it doesn't.
fn is_existing_path(path: &PathBuf) -> bool {
    match fs::exists(&path) {
        Ok(result) => result,
        Err(err) => panic!("There was an error when processing the config file: {err:?}"),
    }
}

/// A serializable struct that holds the config contents data.
#[derive(Serialize, Deserialize)]
pub struct ConfigContents {
    sheet_template_dir: PathBuf,
    sheet_dir: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testdir_path() {
        let path = PathBuf::from("testdir");
        let result = is_existing_path(&path);
        assert_eq!(result, true)
    }

    #[test]
    fn test_path() {
        let path = PathBuf::from("test");
        let result = is_existing_path(&path);
        assert_eq!(result, false)
    }
}
