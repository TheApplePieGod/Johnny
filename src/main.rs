pub mod platform;
pub mod runner;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

fn main() {
    runner::run("test/Program2.json");
}
