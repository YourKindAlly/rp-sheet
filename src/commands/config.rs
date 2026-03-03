/**
 * @license MIT License
 * @author Jasmine Regnér
 */
use crate::config_creator::{create_config_directory};
use inquire::{InquireError, Text};
use std::path::Path;

fn update_config_interactively() {
    let template_sheet = Text::new("Which path do you want to put your sheet templates?").prompt();
}

fn create_input_validator() {
    
}