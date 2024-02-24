pub mod pkl_handling;


use std::process::Command;

// basic function making it easy to call a command to run

pub fn run_command(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&output.stdout).to_string()
}