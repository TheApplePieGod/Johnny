use std::fs;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RunInfo {
    program: String,
    work_dir: String
}

fn main() {
    let run_info_str = fs::read_to_string("test/Program1.json").expect("Failed to load run info data");
    let run_info: RunInfo = serde_json::from_str(&run_info_str).expect("Failed to parse run info data");
    
    println!("Executing process '{}' in directory '{}'", run_info.program, run_info.work_dir);
    let res = Command::new(&run_info.program)
        .current_dir(run_info.work_dir)
        .output()
        .expect("Failed to spawn process");
    println!("Process '{}' completed with {}", &run_info.program, res.status);
}
