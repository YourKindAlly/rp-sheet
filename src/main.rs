/**
 * @license MIT License
 * @author Jasmine Regnér
 */
mod app;
mod config_creator;
mod commands;
mod tests;

use crate::app::{App, get_home_dir};

#[cfg(not(target_os = "linux"))]
compile_error!("This app is only supported on Linux");

fn main() {
    let mut config_path = get_home_dir();
    config_path.push("Documents/rp-sheet");

    let mut template_path = get_home_dir();
    template_path.push("Documents/rp-sheet");

    App::new(&config_path, &template_path);
}
