/**
 * @license MIT License
 * @author Jasmine Regnér
 */
use crate::config_writer::*;
use inquire::{Confirm, Text, validator::Validation};
use regex::Regex;
use std::path::PathBuf;

pub fn update_config_interactively() {
    let regex = match Regex::new(r"^(.*\/)([^\/]*)$") {
        Ok(result) => result,
        Err(err) => {
            println!("There was an error creating the regex: {err:?}");
            return;
        }
    };

    let validator = |input: &str| {
        if regex.is_match(input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("The input is not a valid path.".into()))
        }
    };

    let result = Text::new("In which directory do you want to store your sheet templates?")
        .with_validator(validator)
        .prompt();

    let template_path_input = match result {
        Ok(result) => result,
        Err(err) => {
            println!("There was an error when processing the input: {err:?}");
            return;
        }
    };

    let template_path = PathBuf::from(&template_path_input);

    let result = Text::new("In which directory do you want to store your sheets?")
        .with_validator(validator)
        .prompt();

    let sheet_path_input = match result {
        Ok(result) => result,
        Err(err) => {
            println!("There was an error when processing the input: {err:?}");
            return;
        }
    };

    let sheet_path = PathBuf::from(&sheet_path_input);

    let message = format!(
        "Are these the paths you want to use for storage?\nSheet templates: {template_path:?}\nSheets: {sheet_path:?}"
    );

    let confirmation_input = Confirm::new(&message).with_default(false).prompt();

    match confirmation_input {
        Ok(confirmation) => {
            if !confirmation {
                println!("Aborting config overwrite.");
                return
            }

            let dir_path = create_config_path();
            match create_config_directory(&dir_path) {
                Ok(_result) => {},
                Err(err) => {
                    println!("There was an error when creating config directory: {err:?}");
                }
            }
            
            let contents = ConfigContents::new(template_path, sheet_path);

            overwrite_config_file(&dir_path, &contents);
        },
        Err(err) => {
            println!("There was an error when attempting to overwrite the config. Action aborted: {err:?}");
        }
    }
}

fn overwrite_config_file(dir_path: &PathBuf, contents: &ConfigContents) {
    let file_path = create_config_file_name(dir_path);

    if is_existing_path(&file_path) {
        return;
    }

    let json = create_json(contents);
    write_to_config_file(&file_path, &json);
}

