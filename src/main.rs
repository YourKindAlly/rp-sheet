/**
 * @license MIT License
 * @author Jasmine Regnér
 */

#[cfg(not(target_os = "linux"))]
compile_error!("This app is only supported on Linux");

mod config;

fn main() {
    println!("Hello world!")
}
