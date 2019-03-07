use std::env;

pub fn main() {
    // Allow conditional compilation in debug mode.
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg={}", profile);
    }
}
