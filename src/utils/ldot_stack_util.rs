use std::path::{Path, self};

use crate::{models::stack_config_json::{ProjectConfig, ScriptConfig, StackConfig, StageConfig}, utils::generic_utils};

fn get_default_directory_from_console() -> String {
    let default_dir = generic_utils::get_current_working_dir().unwrap().as_path().display().to_string() + "/ldot_stack.json";
    println!(
        "LDOT stack file name? Default: {}",
        default_dir
    );
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
        std::io::stdin().read_line(&mut stack_name).unwrap();
    }
    if stack_name == "" {
        stack_name = default_stack_name;
    }
    return stack_name.to_string();
}

fn get_stack_version_from_console() -> String {
    let default_stack_version = "stack".to_string();
    println!(
        "Stack version? Default: {}",
        default_stack_version
    );
    let mut stack_version = String::new();
    std::io::stdin().read_line(&mut stack_version).unwrap();
    if stack_version == "" {
        stack_version = default_stack_version;
    }
    return stack_version.to_string();
}

fn get_stack_description_from_console() -> String {
    let default_stack_description = "stack".to_string();
    println!(
        "Stack description? Default: {}",
        default_stack_description
    );
    let mut stack_description = String::new();
    std::io::stdin().read_line(&mut stack_description).unwrap();
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
    let description = get_stack_description_from_console();

    let stack: StackConfig = StackConfig {
        version: ("1.0".to_string()),
        stack_name: ("stack".to_string()),
        description: ("description".to_string()),
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

    Ok(stack)
}

fn validate_ldot_stack(filename: String) {}

fn load_ldot_stack(filename: String) {}

fn unload_ldot_stack(filename: String) {}
