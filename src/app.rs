/**
 * @license MIT License
 * @author Jasmine Regnér
 */
use crate::config_writer::*;
use crate::options::config::update_config_interactively;
use inquire::{InquireError, Select};
use home::home_dir;
use std::collections::HashMap;
use std::path::PathBuf;

/// The rp-sheet application.
pub struct App {
}

impl App {
    /// Creates a new app object.
    pub fn init(config_path: &PathBuf) -> Self {
        match create_config_directory(&config_path) {
            Ok(_result) => {}
            Err(err) => {
                println!("There was an error when processing the config file: {err:?}");
                return Self {};
            }
        }

        let file_path = create_config_file_name(config_path);

        let home_path = match home_dir() {
            Some(result) => result,
            None => {
                panic!("Could not get the home directory.");
            }
        };

        let mut template_path = home_path.clone();
        template_path.push("Documents/rp-tool/templates");

        let mut sheet_path = home_path.clone();
        sheet_path.push("Documents/rp-tool");

        let config = ConfigContents::new(template_path, sheet_path);

        if is_existing_path(&file_path) {
            return Self {};
        }

        let json = create_json(&config);
        write_to_config_file(&file_path, &json);
        Self {}
    }

    pub fn display_options(&self) {
        let mut options: HashMap<String, fn()> = HashMap::new();
        options.insert(
            String::from("Update the config file"),
            update_config_interactively,
        );

        let result = fetch_user_selection(&options);

        let option = match result {
            Ok(result) => result,
            Err(err) => {
                self.display_options();
                println!("There was an error processing the option: {err:?}");
                return;
            }
        };

        let callback_option = options.get(&option);
        let callback = match callback_option {
            Some(result) => result,
            None => {
                println!("No option was selected.");
                self.display_options();
                return;
            }
        };

        callback();
    }
}

pub fn fetch_user_selection(options: &HashMap<String, fn()>) -> Result<String, InquireError> {
    Select::new(
        "What would you like to do?",
        options.keys().cloned().collect(),
    )
    .prompt()
}
