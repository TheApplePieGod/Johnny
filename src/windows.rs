use windows_sys::{Win32::{UI::WindowsAndMessaging::*, Foundation::*}};
use std::thread;
use std::time;
use std::process::Command;

use crate::runner::{TestEvent, KeyCode};

struct FindWindowData {
    pid: u32,
    all_windows: bool,
    handles: Vec<HWND>
}

unsafe extern "system" fn enum_windows_callback(handle: HWND, lparam: LPARAM) -> i32 {
    let data = &mut*(lparam as *mut FindWindowData);
    
    // We only care about windows belonging to the process with data.pid
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(handle, &mut pid);
    if data.pid != pid {
        return 1;
    }

    // We only care about the main window if data.all_windows is false
    if data.all_windows || (GetWindow(handle, GW_OWNER) == 0 && IsWindowVisible(handle) == 1) {
        data.handles.push(handle);
        
        // Continue searching
        if data.all_windows {
            return 1;
        }
    }

    0
}

pub fn get_process_windows(pid: u32, all_windows: bool) -> Option<Vec<isize>> {
    let data = FindWindowData {
        pid,
        all_windows,
        handles: vec!()
    };

    unsafe {
        let data_ptr = &data as *const FindWindowData;
        EnumWindows(Some(enum_windows_callback), data_ptr as isize);
        
        if all_windows {
            for i in 0..data.handles.len() {
                EnumChildWindows(data.handles[i], Some(enum_windows_callback), data_ptr as isize);
            }
        }
    }
    
    match data.handles.len() {
        0 => None,
        _ => Some(data.handles)
    }
}

pub fn kill_process(pid: u32) {
    println!("Attempting to kill process {pid}");
    let status = Command::new("taskkill").args(["/pid", &pid.to_string()]).status();
    match status {
        Ok(code) => {
            if code.success() {
                println!("Process ended successfully");
                return;
            }
        },
        Err(err) => println!("Kill process command failed: {err}")
    }

    println!("Process failed to exit, must be closed manually");
}

fn key_to_wincode(key: &KeyCode) -> usize {
    // Regular keys
    let key_val = *key as usize;
    match key_val {
        0..=9 => return key_val + 48, // Numbers
        10..=35 => return key_val + 55, // Letters
        _ => {}
    }

    // Special keys
    match key {
        KeyCode::LShift => 0xA0,
        KeyCode::LCtrl => 0xA2,
        _ => 0
    }
}

pub fn process_event(windows: &[isize], event: &TestEvent) {
    match event {
        TestEvent::Delay { amount } => thread::sleep(time::Duration::from_millis(*amount)),
        TestEvent::KeyDown { key } => unsafe {
            let win_key = key_to_wincode(key);
            for window in windows {
                SendMessageA(*window, WM_CHAR, win_key, 0);
                SendMessageA(*window, WM_KEYDOWN, win_key, 0);
            }
        },
        TestEvent::KeyUp { key } => unsafe {
            let win_key = key_to_wincode(key);
            let keyup_flags = ((KF_UP | KF_REPEAT | KF_ALTDOWN) << 16) as isize;
            for window in windows {
                SendMessageA(*window, WM_KEYUP, win_key, keyup_flags);
            }
        }
    }
}