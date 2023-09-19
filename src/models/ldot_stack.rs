use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Deserialize)]
struct StackConfig {
    #[serde(default)] // Make version field optional and default to an empty string
    version: String,
    stack_name: String,
    #[serde(default)] // Make description field optional and default to an empty string
    description: String,
    projects: Vec<ProjectConfig>,
}

#[derive(Deserialize)]
struct ProjectConfig {
    project_name: String,
    #[serde(default)] // Make project_description field optional and default to an empty string
    project_description: String,
    stages: Vec<StageConfig>,
}

#[derive(Deserialize)]
struct StageConfig {
    stage_name: String,
    #[serde(default)] // Make stage_description field optional and default to an empty string
    stage_description: String,
    #[serde(default)] // Make prerequisites field optional and default to an empty array
    prerequisites: Vec<String>,
    #[serde(default)] // Make commands field optional and default to an empty array
    commands: Vec<String>,
}

// Deserialize JSON string into StackConfig
fn deserialize_stack_config(json_str: &str) -> Result<StackConfig, serde_json::Error> {
    serde_json::from_str(json_str)
}

// Deserialize JSON string into StackConfig using custom deserialization logic
fn deserialize_stack_config_custom(json_str: &str) -> Result<StackConfig, serde_json::Error> {
    let mut config: StackConfig = serde_json::from_str(json_str)?;

    // Handle the optional fields with default values
    if config.version.is_empty() {
        config.version = "1.0".to_string();
    }
    if config.description.is_empty() {
        config.description = "No description".to_string();
    }

    for project in &mut config.projects {
        // Handle optional project_description with a default value
        if project.project_description.is_empty() {
            project.project_description = "No description".to_string();
        }

        for stage in &mut project.stages {
            // Handle optional stage_description with a default value
            if stage.stage_description.is_empty() {
                stage.stage_description = "No description".to_string();
            }

            // Handle optional prerequisites and commands fields with default empty arrays
            if stage.prerequisites.is_empty() && stage.commands.is_empty() {
                stage.commands.push("No commands specified".to_string());
            }
        }
    }

    Ok(config)
}
