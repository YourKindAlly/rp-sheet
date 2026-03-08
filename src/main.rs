/**
 * @license MIT License
 * @author Jasmine Regnér
 */
mod app;
mod config_writer;
mod options;
mod tests;

use crate::{app::App, config_writer::create_config_path};
use home::home_dir;

#[cfg(not(target_os = "linux"))]
compile_error!("This app is only supported on Linux");

fn main() {
    let config_path = create_config_path();

    let home_path = match home_dir() {
        Some(result) => result,
        None => {
            panic!("Could not get the home directory.");
        }
    };

    let mut template_path = home_path.clone();
    template_path.push("Documents/rp-sheet");

    let app = App::init(&config_path);
    app.display_options();
}
