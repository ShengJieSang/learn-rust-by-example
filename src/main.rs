#![allow(dead_code)]

fn main() {
    #[cfg(target_os = "macos")]
    println!("It's running on Mac OS");

    #[cfg(target_os = "linux")]
    println!("It's running on Linux");
}
