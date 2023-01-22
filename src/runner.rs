use std::fs;
use std::process::Command;
use std::time;
use std::thread;
use serde::{Deserialize, Serialize};
use crate::windows;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum KeyCode {
    Zero = 0,
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    LShift, LCtrl
}

#[derive(Serialize, Deserialize)]
pub enum TestEvent {
    Delay { amount: u64 },
    KeyDown { key: KeyCode, direct: bool },
    KeyUp { key: KeyCode, direct: bool }
}

#[derive(Serialize, Deserialize)]
pub struct RunInfo {
    program: String,
    work_dir: String,
    startup_timeout: u64,
    process_all_windows: bool,
    events: Vec<TestEvent>
}

pub fn run() {
    // Parse run info
    let run_info_str = fs::read_to_string("test/Program2.json").expect("Failed to load run info data");
    let run_info: RunInfo = serde_json::from_str(&run_info_str).expect("Failed to parse run info data");
    
    println!("Executing process '{}' in directory '{}'", run_info.program, run_info.work_dir);

    // Create process
    let process = Command::new(&run_info.program)
        .current_dir(run_info.work_dir)
        .spawn()
        .expect("Failed to spawn process");
    let pid = process.id();
    println!("Process started with pid {}", pid);

    if run_info.startup_timeout > 0 {
        thread::sleep(time::Duration::from_millis(run_info.startup_timeout));
    }
    
    // Wait until the process creates a window and we can find it
    let mut window_handles = None;
    while window_handles.is_none() {
        window_handles = windows::get_process_windows(pid, run_info.process_all_windows);
    }
    let window_handles = window_handles.unwrap();
    
    // Begin executing tests while process is running
    thread::scope(|s| {
        let handle = s.spawn(|| {
            let output = process.wait_with_output().unwrap();
            println!("Process '{}' completed with {}", &run_info.program, output.status);
        });

        for event in &run_info.events {
            if handle.is_finished() {
                break;
            }
            windows::process_event(&window_handles, event);
        }

        windows::kill_process(pid);
    });
    
    println!("Done");
    println!("Test for '{}' finished", &run_info.program);
}
