use crate::runner::TestEvent;

#[cfg(target_os = "windows")]
use crate::windows;

#[cfg(target_os = "macos")]
use crate::macos;

pub fn get_process_windows(pid: u32, all_windows: bool) -> Option<Vec<isize>> {
    #[cfg(target_os = "windows")]
    return windows::get_process_windows(pid, all_windows);

    #[cfg(target_os = "macos")]
    return macos::get_process_windows(pid, all_windows);
}

pub fn kill_process(pid: u32) {
    println!("Attempting to kill process {pid}");

    #[cfg(target_os = "windows")]
    let status = windows::kill_process(pid);

    #[cfg(target_os = "macos")]
    let status = macos::kill_process(pid);

    match status {
        Ok(code) => {
            if code.success() {
                println!("Process ended successfully");
                return;
            }
        }
        Err(err) => println!("Kill process command failed: {err}"),
    }

    println!("Process failed to exit, must be closed manually");
}

pub fn process_event(windows: &[isize], event: &TestEvent) {
    #[cfg(target_os = "windows")]
    windows::process_event(windows, event);

    #[cfg(target_os = "macos")]
    macos::process_event(windows, event);
}
