use std::fs;
use std::path::{Path, PathBuf};
use crate::env;
use serde_json;

use crate::models::ldot_config_json::Configuration;

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
        let default_configuration: Configuration = Configuration {
            default_stack: String::from("stack"),
            registered_stack_files: vec![],
        };

        write_configuration(&default_configuration)?;

        Ok(default_configuration)
    }
}

// Function to write the configuration to the JSON file.
pub fn write_configuration(config: &Configuration) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new(CONFIG_FILE_PATH);
    let config_str = serde_json::to_string_pretty(config)?;
    println!("{}\n contents: \n{}", config_path.to_string_lossy(), config_str);
    fs::write(config_path, config_str)?;
    Ok(())
}

// Function to set the default stack.
pub fn set_default_stack(default_stack: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = get_configuration()?;
    config.default_stack = default_stack.to_string();
    write_configuration(&config)?;
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

// Function to edit the configurations (you can implement this based on your requirements).
pub fn edit_configurations() -> Result<(), Box<dyn std::error::Error>> {
    // Implement your logic for editing configurations.
    Ok(())
}

// Function to regenerate the configuration file.
pub fn regenerate_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let default_config = Configuration {
        default_stack: String::from("stack"),
        registered_stack_files: vec![],
    };
    write_configuration(&default_config)?;
    Ok(())
}
