/**
 * @license MIT License
 * @author Jasmine Regnér
 */
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

/// Creates the config file if one doesn't exist.
fn create_config_file(path: &Path) {
    match create_config_directory(&path) {
        Ok(_result) => {}
        Err(err) => panic!("There was an error when processing the config file: {err:?}"),
    }

    let file_path = create_config_file_name(path);
    let result = is_existing_path(&file_path);
    if result {
        return;
    }
}

/// Creates the directory in which the config file will be created into if it doesn't already exist.
pub fn create_config_directory(path: &Path) -> Result<()> {
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

pub fn create_config_contents(dir_path: &Path) -> ConfigContents {
    let path = format!("{}/templates", dir_path.to_str().unwrap());
    let template_dir_path = PathBuf::from(path);
    ConfigContents::new(template_dir_path)
}

/// A serializable struct that holds the config contents data.
#[derive(Serialize, Deserialize)]
pub struct ConfigContents {
    pub sheet_template_dir: PathBuf,
}

impl ConfigContents {
    pub fn new(sheet_template_dir: PathBuf) -> Self {
        Self { sheet_template_dir }
    }
}
