/**
 * @license MIT License
 * @author Jasmine Regnér
 */

#[cfg(not(target_os = "linux"))]
compile_error!("This app is only supported on Linux");

fn main() {
    println!("Hello world!")
}
