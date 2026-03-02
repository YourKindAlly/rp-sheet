/**
 * @license MIT License
 * @author Jasmine Regnér
 */
use crate::config::*;
use std::path::PathBuf;

/// The rp-sheet application.
pub struct App {
    config: ConfigContents,
}

impl App {
    /// Creates a new app object.
    pub fn new(config_path: &PathBuf, template_path: &PathBuf) -> Self {
        match create_config_directory(&config_path) {
            Ok(_result) => {}
            Err(err) => panic!("There was an error when processing the config file: {err:?}"),
        }

        let file_path = create_config_file_name(config_path);

        let config = create_config_contents(template_path);

        if is_existing_path(&file_path) {
            return Self { config };
        }

        let json = create_json(&config);
        write_to_config_file(&file_path, &json);

        Self { config }
    }
}
