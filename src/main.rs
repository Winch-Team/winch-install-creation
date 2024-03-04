use std::fs::File;
use std::io::{BufReader, Write};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PackageConfig {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    author: String,
    name: String,
    version: String,
    description: String,
    build_steps: Vec<String>,
    dependencies: Vec<String>,
    output_dir: String,
}

fn main() {
    let file = File::open("config.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let config: PackageConfig = serde_json::from_reader(reader).expect("Failed to parse config");

    let mut python_script = String::new();
    python_script.push_str("import os\n");
    python_script.push_str("import subprocess\n");
    python_script.push_str("from colorama import Fore, Style\n");
    python_script.push_str("\n");
    python_script.push_str("# Function to print winch messages with color\n");
    python_script.push_str("def print_winch(message, color=Fore.CYAN):\n");
    python_script.push_str("    print(f'{Style.BRIGHT}{color}(winch) {message}{Style.RESET_ALL}')\n");
    python_script.push_str("\n");
    python_script.push_str("# Function to print step messages with color and formatting\n");
    python_script.push_str("def print_step(step_number, step_description, color=Fore.WHITE):\n");
    python_script.push_str("    print(f'{Style.BRIGHT}{color}  {step_number}. {step_description}{Style.RESET_ALL}')\n");
    python_script.push_str("\n");
    python_script.push_str("# ASCII art for the title\n");
    python_script.push_str("title = r\"\"\"\n");
    python_script.push_str(".%%...%%..%%%%%%..%%..%%...%%%%...%%..%%.\n");
    python_script.push_str(".%%...%%....%%....%%%.%%..%%..%%..%%..%%.\n");
    python_script.push_str(".%%.%.%%....%%....%%.%%%..%%......%%%%%%.\n");
    python_script.push_str(".%%%%%%%....%%....%%..%%..%%..%%..%%..%%.\n");
    python_script.push_str("..%%.%%...%%%%%%..%%..%%...%%%%...%%..%%.\n");
    python_script.push_str(".........................................\n");
    python_script.push_str("\"\"\"\n");
    python_script.push_str("\n");
    python_script.push_str("# Print the title\n");
    python_script.push_str("print(Fore.BLACK + title + Style.RESET_ALL)\n");
    python_script.push_str("\n");
    python_script.push_str("# Example Repo installation steps\n");
    python_script.push_str("install_deps = input('Do you want to install the dependencies? (Y/n): ')\n");
    python_script.push_str("if install_deps.lower() == 'y' or install_deps.lower() == 'yes':\n");
    for dependency in &config.package.dependencies {
        python_script.push_str(&format!("    print_winch('{} requires {} to be installed.')\n", config.package.name, dependency));
        python_script.push_str("    # Add code to install the dependency\n");
    }
    python_script.push_str("    install = input('Do you want to continue with the installation? (Y/n): ')\n");
    python_script.push_str("    if install.lower() == 'y' or install.lower() == 'yes':\n");
    python_script.push_str(&format!("        print_winch('Installing {}...')\n", config.package.name));
    python_script.push_str(&format!("        print_winch('Verifying build steps for {}...')\n", config.package.name));
    python_script.push_str(&format!("        print_winch('{} requires the following build steps:')\n", config.package.name));
    for (index, step) in config.package.build_steps.iter().enumerate() {
        python_script.push_str(&format!("        print_step({}, '{}', Fore.GREEN)\n", index + 1, step));
        python_script.push_str(&format!("        print_winch('Running step: {}')\n", step));
        python_script.push_str(&format!("        subprocess.run(['{}'], shell=True)\n", step));
    }
    python_script.push_str("        print_winch('Installation completed successfully.')\n");
    python_script.push_str("    else:\n");
    python_script.push_str("        print_winch('Installation aborted by user.')\n");

    let output_filename = format!("{}_{}.py", config.package.name.replace(" ", ""), config.package.version);
    let mut output_file = File::create(output_filename).expect("Failed to create output file");
    output_file.write_all(python_script.as_bytes()).expect("Failed to write output file");
}
