use std::process::{Command, Stdio};
use std::vec::Vec;

pub fn active_monitors() -> Vec<String> {
    let mut active_monitors = Vec::new();

    let command = format!("xrandr");

    let child = Command::new(command)
        .arg("--listactivemonitors")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let output = child.wait_with_output().expect("failed to wait on child");
    let output = String::from_utf8_lossy(&output.stdout);

    let mut lines = output.lines();
    lines.next();
    for line in lines {
        let mut line = line.split_whitespace();
        let screen = line.next_back().unwrap();
        active_monitors.push(String::from(screen));
    }

    return active_monitors;
}

pub fn set_brightness(monitor_output: &str, brightness: f32) -> bool {
    let command = format!("xrandr");

    let child = Command::new(command)
        .arg("--output")
        .arg(format!("{}", monitor_output))
        .arg("--brightness")
        .arg(format!("{}", brightness))
        .spawn()
        .expect("failed to execute process");

    child.wait_with_output().expect("failed to wait on child");

    return true;
}
