/**
 * @license MIT License
 * @author Jasmine Regnér
 */
use crate::config_creator::*;
use crate::commands::config::update_config_interactively;
use home::home_dir;
use std::path::PathBuf;
use inquire::{Select, InquireError};
use std::collections::HashMap;

/// The rp-sheet application.
pub struct App {}

impl App {
    /// Creates a new app object.
    pub fn init(config_path: &PathBuf, template_path: &PathBuf) {
        match create_config_directory(&config_path) {
            Ok(_result) => {}
            Err(err) => {
                println!("There was an error when processing the config file: {err:?}");
                return
            },
        }

        let file_path = create_config_file_name(config_path);
        let config = create_config_contents(template_path);

        if is_existing_path(&file_path) {
            return
        }

        let json = create_json(&config);
        write_to_config_file(&file_path, &json);
    }

    fn display_options(&self) {
        let mut options: HashMap<String, fn()> = HashMap::new();
        options.insert(String::from("Update the config file"), update_config_interactively);

        let result = fetch_user_selection(&options);

        let option = match result {
            Ok(result) => result,
            Err(err) => {
                self.display_options();
                println!("There was an error processing the option: {err:?}");
                return
            }
        };
        
        let callback_option = options.get(&option);
        let callback = match callback_option {
            Some(result) => result,
            None => {
                println!("No option was selected.");
                self.display_options();
                return
            }
        };

        callback();
    }
}

pub fn get_home_dir() -> PathBuf {
    match home_dir() {
        Some(result) => result,
        None => {
            panic!("Couldn't find user's home directory.")
        }
    }
}

pub fn fetch_user_selection(options: &HashMap<String, fn()>) -> Result<String, InquireError> {
    Select::new("What would you like to do?", options.keys().cloned().collect()).prompt()
}