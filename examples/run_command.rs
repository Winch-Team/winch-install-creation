use winch_install_creation::*;

fn main() {
    // run a command that creates a file in the current directory
    let command = "touch";
    let args = &["test_file"];

    match run_command(command, args) {
        Ok(_) => println!("Command ran successfully"),
        Err(e) => println!("Error running command: {}", e),
    }
}
