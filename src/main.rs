use std::fs::File;
use std::io::{BufReader, Write};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PackageConfig {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    build_steps: Vec<String>,
}

fn main() {
    let file = File::open("config.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let config: PackageConfig = serde_json::from_reader(reader).expect("Failed to parse config");

    let mut python_script = String::from("import os\n");

    for step in config.package.build_steps {
        python_script.push_str(&format!("\nprint('Running step: {}')\n", step));
        python_script.push_str(&format!("os.system('{}')\n", step));
    }

    let output_filename = format!("{}_{}.py", config.package.name, config.package.version);
    let mut output_file = File::create(output_filename).expect("Failed to create output file");
    output_file.write_all(python_script.as_bytes()).expect("Failed to write output file");
}
