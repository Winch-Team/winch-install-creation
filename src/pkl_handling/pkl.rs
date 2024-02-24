use crate::run_command;

pub fn pkltojson(file: &str) -> String {

    let output = run_command("pkl", &["eval", "-f", "json", file]);
    
    println!("{}", output);

    output
}