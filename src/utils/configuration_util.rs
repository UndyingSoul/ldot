use serde_json;
use std::fs;
use std::path::Path;

use crate::models::ldot_config_json::Configuration;
use crate::models::stack_config_json::StackConfig;
use crate::utils::ldot_stack_util;

// Define the path to your JSON configuration file.
const CONFIG_FILE_PATH: &str = "./data/config.json";

// Function to read the configuration from the JSON file.
pub fn get_configuration() -> Result<Configuration, Box<dyn std::error::Error>> {
    let config_path = Path::new(CONFIG_FILE_PATH);
    if config_path.exists() {
        let config_str = fs::read_to_string(config_path)?;
        let config: Configuration = serde_json::from_str(&config_str)?;
        Ok(config)
    } else {
        // Return a default configuration or handle the missing file as needed.
        println!(
            "Could not fetch current configuration at: {}",
            config_path.to_string_lossy()
        );
        let default_configuration: Configuration = Configuration {
            default_stack: String::from(""),
            registered_stack_files: vec![],
        };

        println!(
            "Regenerating configuration at: {}",
            config_path.to_string_lossy()
        );
        write_configuration(&default_configuration)?;

        Ok(default_configuration)
    }
}

// Function to write the configuration to the JSON file.
pub fn write_configuration(config: &Configuration) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new(CONFIG_FILE_PATH);
    let config_str = serde_json::to_string_pretty(config)?;
    //println!("{}\n contents: \n{}", config_path.to_string_lossy(), config_str);
    fs::write(config_path, config_str)?;
    Ok(())
}

pub fn validate_stack_names(stack_name: &str) -> Result<&str, String> {
    let config = get_configuration().unwrap();

    let mut valid_stack_names: Vec<String> = vec!["".to_string()];
    for stack_file in &config.registered_stack_files {
        let json_str: String;
        match std::fs::read_to_string(stack_file) {
            Ok(stack_file_json_string) => {
                json_str = stack_file_json_string;
            }
            Err(err) => {
                eprintln!("Error opening stack file: {} {}", stack_file, err);
                continue;
            }
        }

        let stack_config: StackConfig;
        match serde_json::from_str(&json_str) {
            Ok(stack_object) => {
                stack_config = stack_object;
            }
            Err(err) => {
                eprintln!(
                    "Could not convert stack file to valid configuration: {} {}",
                    stack_file, err
                );
                continue;
            }
        }

        valid_stack_names.push(stack_config.stack_name);
    }

    if valid_stack_names
        .iter()
        .any(|valid_stack_name| stack_name.to_string() == valid_stack_name.to_string())
    {
        Ok(stack_name)
    } else {
        Err(format!(
            "Could not find stack name in registered stack files: {}",
            stack_name.to_string()
        ))
    }
}

// Function to set the default stack.
pub fn set_default_stack(desired_stack: &str) -> Result<(), Box<dyn std::error::Error>> {
    match validate_stack_names(desired_stack) {
        Ok(_) => {
            let mut config = get_configuration()?;
            config.default_stack = desired_stack.to_string();
            write_configuration(&config)?;
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
    Ok(())
}

// Function to list the registered configurations.
pub fn list_configurations() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_configuration()?;
    println!("Default Stack: {}", config.default_stack);
    println!("Registered Stack Files:");
    for file in &config.registered_stack_files {
        println!("    {}", file);
    }
    Ok(())
}

pub fn get_ldot_stack_from_stack_name(stack_name: &str) -> Result<String, String> {
    let mut ldot_stack_file = "".to_string();

    for stack_file in get_configuration().unwrap().registered_stack_files {
        let latest_stack_file = stack_file;
        match ldot_stack_util::validate_ldot_stack(latest_stack_file.to_string()) {
            Ok(stack_config) => {
                if stack_config.stack_name == stack_name {
                    ldot_stack_file = latest_stack_file.to_string();
                    break;
                }
            }
            Err(err) => {
                eprintln!("{} ", err);
                continue;
            }
        }
    }
    if ldot_stack_file != "" {
        Ok(ldot_stack_file.to_string())
    } else {
        Err(format!(
            "Could not find any stack files that have a stack name of: {}",
            stack_name.to_string()
        ))
    }
}

// // Function to edit the configurations (you can implement this based on your requirements).
// pub fn edit_configurations() -> Result<(), Box<dyn std::error::Error>> {
//     // Implement your logic for editing configurations.
//     Ok(())
// }

// Function to regenerate the configuration file.
pub fn regenerate_configuration() -> Result<Configuration, Box<dyn std::error::Error>> {
    let default_config = Configuration {
        default_stack: String::from(""),
        registered_stack_files: vec![],
    };
    write_configuration(&default_config)?;
    Ok(default_config)
}
