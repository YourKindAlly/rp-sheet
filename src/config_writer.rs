/**
 * @license MIT License
 * @author Jasmine Regnér
 */
use serde::{Deserialize, Serialize};
use home::home_dir;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

/// Gets path to the config file in "/home/user/Documents/rp_tool"
pub fn create_config_path() -> PathBuf {
    let home_path = match home_dir() {
        Some(result) => result,
        None => {
            panic!("Could not get the home directory.");
        }
    };

    let mut config_path = home_path.clone();
    config_path.push("Documents/rp_tool");
    config_path
}

/// Creates the directory in which the config file will be created into if it doesn't already exist.
pub fn create_config_directory(path: &PathBuf) -> Result<()> {
    if is_existing_path(path) {
        return Ok(());
    }

    fs::create_dir_all(path)?;
    Ok(())
}

/// Takes the path directory and adds config.json to the path.
pub fn create_config_file_name(dir_path: &Path) -> PathBuf {
    let mut file_path: PathBuf = PathBuf::from(dir_path);
    file_path.push("config");
    file_path.set_extension("json");
    return file_path;
}

/// Returns true if the path exists, and false if it doesn't. Panics if neither can be returned.
pub fn is_existing_path(path: &Path) -> bool {
    match fs::exists(&path) {
        Ok(result) => result,
        Err(err) => panic!("There was an error when processing the config file: {err:?}"),
    }
}

/// Returns a json formatted string from an object reference of ConfigContents
pub fn create_json(config: &ConfigContents) -> String {
    match serde_json::to_string(config) {
        Ok(result) => result,
        Err(err) => panic!("There was an error when creating the config contents: {err:?}"),
    }
}

/// Create and write to the config file in given directory using the json string. Panics if there is an error to create the file.
pub fn write_to_config_file(path: &PathBuf, contents: &String) -> () {
    match fs::write(path, contents) {
        Ok(_result) => (),
        Err(err) => panic!("There was an error when creating the config file: {err:?}"),
    }
}

/// A serializable struct that holds the config contents data.
#[derive(Serialize, Deserialize)]
pub struct ConfigContents {
    pub sheet_template_dir: PathBuf,
    pub sheet_dir: PathBuf,
}

impl ConfigContents {
    pub fn new(sheet_template_dir: PathBuf, sheet_dir: PathBuf) -> Self {
        Self {
            sheet_template_dir,
            sheet_dir,
        }
    }
}
