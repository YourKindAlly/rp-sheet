use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
/**
 * @license MIT License
 * @author Jasmine Regnér
 */

/// Ensures that the directory exists or is created. Panics if there is an error in the process.
fn create_config_file(path: &Path) {
    match create_config_directory(&path) {
        Ok(_result) => {}
        Err(err) => panic!("There was an error when processing the config file: {err:?}"),
    }

    let dir_path = create_config_file_name(path);
}

/// Creates the directory in which the config file will be created into if it doesn't already exist.
fn create_config_directory(path: &Path) -> Result<()> {
    if is_existing_path(path) {
        return Ok(());
    }

    fs::create_dir_all(path)?;
    Ok(())
}

fn create_config_file_name(dir_path: &Path) -> PathBuf {
    let mut file_path: PathBuf = PathBuf::from(dir_path);
    file_path.push("config");
    file_path.set_extension("json");
    return file_path;
}

/// Returns true if the path exists, and false if it doesn't. Panics if neither can be returned.
fn is_existing_path(path: &Path) -> bool {
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
    fn test_create_config() {
        let path = Path::new("testdirs/user");
        let result = create_config_directory(&path).unwrap();
        assert_eq!(result, ());

        let path = Path::new("testdirs/admin");
        let result = create_config_directory(&path).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn test_path_exists() {
        let path = Path::new("testdirs/user");
        let result = is_existing_path(&path);
        assert_eq!(result, true);

        let path = Path::new("testdirs/admin");
        let result = is_existing_path(&path);
        assert_eq!(result, true);

        let path = Path::new("testdirs/empty");
        let result = is_existing_path(&path);
        assert_eq!(result, false);
    }

    #[test]
    fn test_file_name() {
        let path = Path::new("testdirs/user");
        let result = create_config_file_name(path);
        assert_eq!(result, PathBuf::from("testdirs/user/config.json"));
    }
}
