use core_graphics::event::*;
use core_graphics::event_source::*;
use std::io;
use std::process;
use std::thread;
use std::time;

use crate::runner::{KeyCode, TestEvent};

pub fn get_process_windows(pid: u32, all_windows: bool) -> Option<Vec<isize>> {
    Some(vec![pid as isize])
}

pub fn kill_process(pid: u32) -> Result<process::ExitStatus, io::Error> {
    process::Command::new("kill")
        .args([&pid.to_string()])
        .status()
}

pub fn process_event(windows: &[isize], event: &TestEvent) {
    match event {
        TestEvent::Delay { amount } => thread::sleep(time::Duration::from_millis(*amount)),
        TestEvent::KeyDown { key, direct } => {
            let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState).unwrap();
            let kevent = CGEvent::new_keyboard_event(source, 0, true).unwrap();
            kevent.post_to_pid(windows[0] as i32);
        }
        TestEvent::KeyUp { key, direct } => {}
        TestEvent::MouseDown { button, direct } => {}
        TestEvent::MouseUp { button, direct } => {}
    }
}
