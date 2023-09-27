use serde::{Deserialize, Serialize, ser::SerializeStruct};

#[derive(Deserialize)]
pub struct StackConfig {
    #[serde(default)] // Make version field optional and default to an empty string
    pub version: String,
    pub stack_name: String,
    #[serde(default)] // Make description field optional and default to an empty string
    pub description: String,
    pub projects: Vec<ProjectConfig>,
    pub scripts: Vec<ScriptConfig>,
}

#[derive(Deserialize)]
pub struct ProjectConfig {
    pub project_name: String,
    #[serde(default)] // Make project_description field optional and default to an empty string
    pub project_description: String,
    pub stages: Vec<StageConfig>,
}

#[derive(Deserialize)]
pub struct StageConfig {
    pub stage_name: String,
    #[serde(default)] // Make stage_description field optional and default to an empty string
    pub stage_description: String,
    #[serde(default)] // Make prerequisites field optional and default to an empty array
    pub prerequisites: Vec<String>,
    #[serde(default)] // Make commands field optional and default to an empty array
    pub commands: Vec<String>,
}
#[derive(Deserialize)]
pub struct ScriptConfig {
    pub script_name: String,
    #[serde(default)] // Make project_description field optional and default to an empty string
    pub script_description: String,
    pub commands: Vec<String>,
}
// // Deserialize JSON string into StackConfig
// pub fn deserialize_stack_config(json_str: &str) -> Result<StackConfig, serde_json::Error> {
//     serde_json::from_str(json_str)
// }

// // Deserialize JSON string into StackConfig using custom deserialization logic
// pub fn deserialize_stack_config_custom(json_str: &str) -> Result<StackConfig, serde_json::Error> {
//     let mut config: StackConfig = serde_json::from_str(json_str)?;

//     // Handle the optional fields with default values
//     if config.version.is_empty() {
//         config.version = "1.0".to_string();
//     }
//     if config.description.is_empty() {
//         config.description = "No description".to_string();
//     }

//     for project in &mut config.projects {
//         // Handle optional project_description with a default value
//         if project.project_description.is_empty() {
//             project.project_description = "No description".to_string();
//         }

//         for stage in &mut project.stages {
//             // Handle optional stage_description with a default value
//             if stage.stage_description.is_empty() {
//                 stage.stage_description = "No description".to_string();
//             }

//             // Handle optional prerequisites and commands fields with default empty arrays
//             if stage.prerequisites.is_empty() && stage.commands.is_empty() {
//                 stage.commands.push("No commands specified".to_string());
//             }
//         }
//     }

//     Ok(config)
// }

impl Serialize for StackConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("StackConfig", 5)?;

        state.serialize_field("version", &self.version)?;
        state.serialize_field("stack_name", &self.stack_name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("projects", &self.projects)?;
        state.serialize_field("scripts", &self.scripts)?;

        state.end()
    }
}

// Implement Serialize trait for ProjectConfig
impl Serialize for ProjectConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ProjectConfig", 2)?;

        state.serialize_field("project_name", &self.project_name)?;
        state.serialize_field("project_description", &self.project_description)?;
        state.serialize_field("stages", &self.stages)?;

        state.end()
    }
}

// Implement Serialize trait for StageConfig
impl Serialize for StageConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("StageConfig", 4)?;

        state.serialize_field("stage_name", &self.stage_name)?;
        state.serialize_field("stage_description", &self.stage_description)?;
        state.serialize_field("prerequisites", &self.prerequisites)?;
        state.serialize_field("commands", &self.commands)?;

        state.end()
    }
}

// Implement Serialize trait for ScriptConfig
impl Serialize for ScriptConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ScriptConfig", 2)?;

        state.serialize_field("script_name", &self.script_name)?;
        state.serialize_field("script_description", &self.script_description)?;
        state.serialize_field("commands", &self.commands)?;

        state.end()
    }
}

// Serialize StackConfig to JSON and write it to a file
pub fn serialize_stack_config_to_file(config: &StackConfig, file_path: &str) -> Result<(), std::io::Error> {
    let json_str = serde_json::to_string(config)?;

    // Write the JSON string to the file
    std::fs::write(file_path, json_str)?;

    Ok(())
}





