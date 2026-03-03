/**
 * @license MIT License
 * @author Jasmine Regnér
 */
use crate::config_creator::*;
use crate::commands::config::*;
use clap::{Parser, Subcommand};
use home::home_dir;
use std::path::PathBuf;

pub fn get_home_dir() -> PathBuf {
    match home_dir() {
        Some(result) => result,
        None => {
            panic!("Couldn't find user's home directory.")
        }
    }
}

/// The rp-sheet application.
#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "The CLI application to create sheet and sheet templates for TTRPGs."
)]
pub struct App {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short)]
    args: String
}

impl App {
    /// Creates a new app object.
    pub fn init(&self, config_path: &PathBuf, template_path: &PathBuf) {
        match create_config_directory(&config_path) {
            Ok(_result) => {}
            Err(err) => panic!("There was an error when processing the config file: {err:?}"),
        }

        let file_path = create_config_file_name(config_path);
        let config = create_config_contents(template_path);

        if is_existing_path(&file_path) {
            return
        }

        let json = create_json(&config);
        write_to_config_file(&file_path, &json);
    }

    pub fn process_command(&self) {
        match self.command {
            Some(Commands::Config) => update_config_interactively(),
            None => return,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    Config,
}

