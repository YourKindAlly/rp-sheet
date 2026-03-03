/**
 * @license MIT License
 * @author Jasmine Regnér
 */
use crate::config_creator::create_config_directory;
use inquire::{Confirm, Text, validator::Validation};
use std::path::Path;
use regex::Regex;

pub fn update_config_interactively() {
    let regex = match Regex::new(r"^(.*\/)([^\/]*)$") {
        Ok(result) => result,
        Err(err) => {
            println!("There was an error creating the regex: {err:?}");
            return
        }
    };

    let validator = | input: &str | if regex.is_match(input) {
        Ok(Validation::Valid)
    } else {
        Ok(Validation::Invalid("The input is not a valid path.".into()))
    };

    let result = Text::new("Which path do you want to put your sheet templates?").with_validator(validator).prompt();

    let path_input = match result {
        Ok(result) => result,
        Err(err) => {
            println!("There was an error when processing the input: {err:?}");
            return
        }
    };

    let path = Path::new(&path_input);

    let confirmation_input = Confirm::new("Is this the path you want templates to be stored? {path}").with_default(false).prompt();

    let confirmation = match confirmation_input {
        Ok(result) => result,
        Err(err) => {
            println!("There was an error when processing the confirmation: {err:?}");
            return
        }
    };

    match confirmation {
        true => {
            match create_config_directory(path) {
                Ok(_result) => println!("Updated template sheet directory."),
                Err(err) => {
                    println!("There was an error saving the template sheet directory: {err:?}");
                    return
                }
            }
        },
        false => {
            println!("Aborting updating the config.");
            return
        },
    }
}