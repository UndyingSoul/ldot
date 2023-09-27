use colored::Colorize;

use crate::models::ldot_config_json::Configuration;
use crate::models::stack_config_json::serialize_stack_config_to_file;
use crate::{
    models::stack_config_json::{ProjectConfig, ScriptConfig, StackConfig, StageConfig},
    utils::generic_utils,
};
use std::process::Command;

use super::configuration_util;

use std::fs;

fn get_default_directory_from_console() -> String {
    let default_dir = generic_utils::get_current_working_dir()
        .unwrap()
        .as_path()
        .display()
        .to_string()
        + "/ldot_stack.json";
    println!("LDOT stack file name? Default: {}", default_dir);
    let mut dir = generic_utils::get_file_name_check_if_parent_dir_exists();
    if dir == "" {
        dir = default_dir;
    }
    return dir.to_string();
}

fn get_stack_name_from_console() -> String {
    let default_stack_name = "stack".to_string();
    println!(
        "Stack name (spaces are not allowed)? Default: {}",
        default_stack_name
    );
    let mut stack_name = " ".to_string();
    while stack_name.contains(" ") {
        stack_name = generic_utils::get_line_from_console_allow_blank();
    }
    if stack_name == "" {
        stack_name = default_stack_name;
    }
    return stack_name.to_string();
}

fn get_stack_version_from_console() -> String {
    let default_stack_version = "1.0.0".to_string();
    println!("Stack version? Default: {}", default_stack_version);
    let mut stack_version = generic_utils::get_line_from_console_allow_blank();
    if stack_version == "" {
        stack_version = default_stack_version;
    }
    return stack_version.to_string();
}

fn get_stack_description_from_console() -> String {
    let default_stack_description = "Stack Description".to_string();
    println!("Stack description? Default: {}", default_stack_description);
    let mut stack_description = generic_utils::get_line_from_console_allow_blank();
    if stack_description == "" {
        stack_description = default_stack_description;
    }
    return stack_description.to_string();
}

pub fn generate_ldot_stack() -> Result<StackConfig, String> {
    //default directory
    let dir = get_default_directory_from_console();
    let stack_name = get_stack_name_from_console();
    let stack_version = get_stack_version_from_console();
    let stack_description = get_stack_description_from_console();
    println!("Adding generic projects and scripts to your stack file, you can edit this directly");
    println!("Once you're done editing the file, load it with ldot load");

    let stack_config: StackConfig = StackConfig {
        version: (stack_version),
        stack_name: (stack_name),
        description: (stack_description),
        projects: (vec![ProjectConfig {
            project_name: "some_project".to_string(),
            project_description: "some project description".to_string(),
            stages: vec![StageConfig {
                stage_name: "stage_name".to_string(),
                stage_description: "stage description".to_string(),
                prerequisites: vec![],
                commands: vec!["echo hello world".to_string()],
            }],
        }]),
        scripts: (vec![ScriptConfig {
            script_name: ("script_name".to_string()),
            script_description: ("script description".to_string()),
            commands: (vec!["echo hello world".to_string()]),
        }]),
    };

    if let Err(err) = serialize_stack_config_to_file(&stack_config, &dir) {
        eprintln!("Error serializing and writing to file: {}", err);
    }
    Ok(stack_config)
}

pub fn validate_ldot_stack(filename: String) -> Result<StackConfig, Box<dyn std::error::Error>> {
    // Read the JSON content from the file
    let json_str = std::fs::read_to_string(filename)?;

    // Deserialize the JSON into a StackConfig
    let stack_config: StackConfig = serde_json::from_str(&json_str)?;

    // Perform validation checks here
    let mut unique_project_names: Vec<String> = Vec::new();
    let mut unique_script_names: Vec<String> = Vec::new();
    if stack_config.stack_name.is_empty() {
        return Err("Stack name is empty".into());
    }
    if stack_config.stack_name.contains(" ") {
        return Err("Stack name contains a space".into());
    }

    for project in &stack_config.projects {
        if project.project_name.is_empty() {
            return Err("Project name is empty".into());
        }
        if project.project_name.contains(" ") {
            return Err(format!("Project name contains a space: {}", project.project_name).into());
        }
        if unique_project_names.iter().any(|unique_project_name| {
            project.project_name.to_string() == unique_project_name.to_string()
        }) {
            return Err(format!("Project name not unique: {}", project.project_name).into());
        }
        unique_project_names.push(project.project_name.to_string());
    }

    for script in &stack_config.scripts {
        if script.script_name.is_empty() {
            return Err("Script name is empty".into());
        }
        if script.script_name.contains(" ") {
            return Err(format!("Script name contains a space: {}", script.script_name).into());
        }
        if unique_script_names.iter().any(|unique_script_name| {
            script.script_name.to_string() == unique_script_name.to_string()
        }) {
            return Err(format!("Script name not unique: {}", script.script_name).into());
        }
        unique_script_names.push(script.script_name.to_string());
    }

    // You can add more validation logic as needed

    Ok(stack_config)
}

pub fn load_ldot_stack(filename: String) {
    // Validation logic
    let filename_absolute: String;
    match fs::canonicalize(filename.to_string()) {
        Ok(file) => {
            filename_absolute = file.as_path().display().to_string();
        }
        Err(err) => {
            eprintln!("Could not convert filename to canonical name: {}", err);
            std::process::exit(1);
        }
    }

    match validate_ldot_stack(filename_absolute.to_string()) {
        Ok(_) => {
            println!("Validation succeeded.");
        }
        Err(err) => {
            eprintln!("Validation failed: {}", err);
        }
    }

    let mut ldot_configuration: Configuration;
    match configuration_util::get_configuration() {
        Ok(config) => {
            ldot_configuration = config;
            println!("Retrieved configuration.");
        }
        Err(err) => {
            eprintln!("Could not retrieve configuration: {}", err);
            std::process::exit(1);
        }
    }

    if ldot_configuration
        .registered_stack_files
        .iter()
        .any(|stack_file| filename_absolute.to_string() == stack_file.to_string())
    {
        eprintln!(
            "Stack file name not unique: {}",
            filename_absolute.to_string()
        );
        std::process::exit(1);
    }

    ldot_configuration
        .registered_stack_files
        .push(filename_absolute.to_string());

    match configuration_util::write_configuration(&ldot_configuration) {
        Ok(_) => {
            println!("Updated configuration.");
        }
        Err(err) => {
            eprintln!("Could not update configuration: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn unload_ldot_stack(filename: String) {
    // Validation logic
    let filename_absolute: String;
    match fs::canonicalize(filename.to_string()) {
        Ok(file) => {
            filename_absolute = file.as_path().display().to_string();
        }
        Err(err) => {
            eprintln!("Could not convert filename to canonical name: {}", err);
            std::process::exit(1);
        }
    }

    let mut ldot_configuration: Configuration;
    match configuration_util::get_configuration() {
        Ok(config) => {
            ldot_configuration = config;
            println!("Retrieved configuration.");
        }
        Err(err) => {
            eprintln!("Could not retrieve configuration: {}", err);
            std::process::exit(1);
        }
    }

    if ldot_configuration
        .registered_stack_files
        .iter()
        .any(|stack_file| filename_absolute.to_string() == stack_file.to_string())
    {
        ldot_configuration
            .registered_stack_files
            .retain(|x| x != &filename_absolute.to_string());
        eprintln!("Stack file unloaded: {}", filename_absolute.to_string())
    } else {
        eprintln!(
            "Could not find stack file in configuration: {}",
            filename_absolute.to_string()
        );
        std::process::exit(1);
    }

    match configuration_util::write_configuration(&ldot_configuration) {
        Ok(_) => {
            println!("Updated configuration.");
        }
        Err(err) => {
            eprintln!("Could not update configuration: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn get_ldot_stack_config(ldot_stack_file: String) -> StackConfig {
    // Read the JSON content from the file
    let json_str: String;
    match std::fs::read_to_string(&ldot_stack_file) {
        Ok(file_string) => {
            json_str = file_string;
        }
        Err(e) => {
            eprintln!(
                "Could not open and read stack file: {} {}",
                ldot_stack_file, e
            );
            std::process::exit(2);
        }
    }

    let stack_config: StackConfig;
    match serde_json::from_str(&json_str) {
        Ok(json_str_parsed) => {
            stack_config = json_str_parsed;
        }
        Err(e) => {
            eprintln!(
                "Could not parse stack file to JSON: {} {}",
                ldot_stack_file, e
            );
            std::process::exit(2);
        }
    }
    return stack_config;
}

pub fn execute_stack_stage_script(
    stack: String,
    project: String,
    stage: String,
) -> Result<Vec<std::string::String>, Vec<std::string::String>> {
    let ldot_stack_file_name: String;
    match configuration_util::get_ldot_stack_from_stack_name(&stack) {
        Ok(filename) => {
            ldot_stack_file_name = filename;
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    let ldot_stack_config = get_ldot_stack_config(ldot_stack_file_name);

    for stack_project in ldot_stack_config.projects {
        if stack_project.project_name == project {
            for project_stage in stack_project.stages {
                if project_stage.stage_name == stage {
                    println!("Executing {} commands", project_stage.commands.len());
                    return execute_command_from_list(project_stage.commands);
                }
            }
        }
    }

    // If no matching script is found, you should return an error here.
    Err(vec!["Stage not found".to_string()])
}

pub fn execute_stack_script(
    stack: String,
    script_name: String,
) -> Result<Vec<std::string::String>, Vec<std::string::String>> {
    let ldot_stack_file_name: String;
    match configuration_util::get_ldot_stack_from_stack_name(&stack) {
        Ok(filename) => {
            ldot_stack_file_name = filename;
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    let ldot_stack_config = get_ldot_stack_config(ldot_stack_file_name);
    for script in ldot_stack_config.scripts {
        if script.script_name == script_name {
            println!("Executing {} commands", script.commands.len());
            return execute_command_from_list(script.commands);
        };
    };
    
    // If no matching script is found, you should return an error here.
    Err(vec!["Script not found".to_string()])
}

fn execute_command_from_list(commands: Vec<String>) -> Result<Vec<String>, Vec<String>> {
    let mut cmd_and_code: Vec<String> = vec![];
    let mut errors_occured = false;

    for command in commands {
        let mut args: Vec<String> = command.split(" ").map(|s| s.to_string()).collect();
        let system_command = args[0].to_string();

        args.remove(0);
        let mut cmd_execution = Command::new(&system_command);
        cmd_execution.args(&args);
        let happy_status_code: i32 = 0;
        println!("\n> {}", command.purple());
        match cmd_execution.status() {
            Ok(exit_status) => {
                if Some(exit_status.code()) == Some(Some(happy_status_code)) {
                    cmd_and_code.push(
                        format!(
                            "\"{}\" exit code: {:?}",
                            &command,
                            exit_status.code().unwrap()
                        )
                        .bright_green()
                        .to_string(),
                    )
                } else {
                    cmd_and_code.push(
                        format!(
                            "\"{}\" exit code: {:?}",
                            &command,
                            exit_status.code().unwrap()
                        )
                        .yellow()
                        .to_string(),
                    );
                    errors_occured = true;
                }
            }
            Err(err) => {
                println!("{}", format!("{}", err).bright_red());
                cmd_and_code.push(
                    format!("\"{}\": {}", &command, err)
                        .bright_red()
                        .to_string(),
                );
            }
        }
    }
    if errors_occured {
        Err(cmd_and_code)
    } else {
        Ok(cmd_and_code)
    }
}