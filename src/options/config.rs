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

    let confirmation = match confirmation_input {
        Ok(result) => result,
        Err(err) => {
            println!("There was an error when processing the confirmation: {err:?}");
            return;
        }
    };

    match confirmation {
        true => match create_config_directory(&template_path) {
            Ok(_result) => {
                println!("Updated template sheet directory.")
            }
            Err(err) => {
                println!("There was an error saving the template sheet directory: {err:?}");
                return;
            }
        },
        false => {
            println!("Aborting updating the config.");
            return;
        }
    }
}
