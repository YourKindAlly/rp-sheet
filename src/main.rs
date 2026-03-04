/**
 * @license MIT License
 * @author Jasmine Regnér
 */
mod app;
mod config_writer;
mod options;
mod tests;

use crate::app::App;
use home::home_dir;

#[cfg(not(target_os = "linux"))]
compile_error!("This app is only supported on Linux");

fn main() {
    let home_path = match home_dir() {
        Some(result) => result,
        None => {
            println!("Could not get the home directory.");
            return;
        }
    };

    let mut config_path = home_path.clone();
    config_path.push("Documents/rp-sheet");

    let mut template_path = home_path.clone();
    template_path.push("Documents/rp-sheet");

    let app = App::init(&config_path, &template_path);
    app.display_options();
}
