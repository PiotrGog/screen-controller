use std::process::Command;

pub fn set_brightness(monitor_output: &str, brightness: f32) -> bool {
    let command = format!("./script.sh");

    Command::new(command)
        .arg(format!("{}", monitor_output))
        .arg(format!("{}", brightness))
        .spawn()
        .expect("failed to execute process");

    return true;
}
