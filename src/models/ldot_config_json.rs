use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub default_stack: String,
    pub registered_stack_files: Vec<String>,
}