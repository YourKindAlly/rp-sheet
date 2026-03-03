/**
 * @license MIT License
 * @author Jasmine Regnér
 */
use crate::config_creator::{create_config_directory};
use inquire::Text;

fn overwrite_config_interactively() {
    let template_sheet = Text::new("Where do you want to place your sheet templates?").prompt();
}
