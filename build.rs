use std::env;

pub fn main() {
    // Allow conditional compilation in different modes, for example:
    //
    // ```rust
    // #[cfg(debug)]
    // println!("I'm in debug mode!");
    // #[cfg(release)]
    // println!("I'm in release mode!");
    // ```
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg={}", profile);
    }
}
