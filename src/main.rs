extern crate clap;
mod models {
    pub mod command_line; // This should match your file structure.
    pub mod ldot_config_json;
    pub mod stack_config_json;
}
mod utils {
    pub mod configuration_util;
    pub mod ldot_stack_util;
    mod generic_utils;
}

use std::{env, path::PathBuf};

use clap::Parser;
use models::command_line::{Cli, Commands, ConfigArgs};

use crate::utils::{configuration_util, ldot_stack_util};

fn main() {
    let cli = Cli::parse();
    
    // Handle subcommands and arguments
    match cli.command {
        Commands::Validate(args) => {
            let filename: PathBuf = args.file.unwrap().to_path_buf();
            println!("Validating configuration file: {}", filename.to_string_lossy());
            // Implement the validation logic
        }
        Commands::Generate(args) => {
            let filename: PathBuf = args.file.unwrap().to_path_buf();
            println!("Generating configuration file: {}", filename.to_string_lossy());
            let _ = ldot_stack_util::generate_ldot_stack();
        }
        Commands::Load(args) => {
            let filename: PathBuf = args.file.unwrap().to_path_buf();
            println!("Loading configuration file: {}", filename.to_string_lossy());
            // Implement the loading logic
        }
        Commands::Unload(args) => {
            let filename: PathBuf = args.file.unwrap().to_path_buf();
            println!("Unloading configuration file: {}", filename.to_string_lossy());
            // Implement the loading logic
        }
        Commands::Config(config_command) => match config_command.subcommand {
            ConfigArgs::List(_) => {
                println!("--- Listing Config ---");
                let _ = configuration_util::list_configurations();
            }
            ConfigArgs::Default(default_args) => {
                if let Some(stack) = default_args.stack {
                    println!("Setting default stack to: {}", stack.to_string_lossy());

                    let _ = configuration_util::set_default_stack(&stack.to_string_lossy());
                } else {
                    println!("No default stack specified");
                }
            }
            ConfigArgs::Edit(_) => {
                println!("Prints the config location");
                let _ = configuration_util::edit_configurations();
            }
            ConfigArgs::Regenerate(_) => {
                println!("Regenerating the LDOT config file");
                let _ = configuration_util::regenerate_configuration();
            }
        }
        Commands::Execute(_execute_args) => {
            let args: Vec<String> = env::args().collect();
            println!("Executing");
            // Handle the case where stack/project/stage names are provided
            if args.len() == 4 {
                let stack_name = "DEFAULT_STACK_NAME";
                let project_name = args.get(2);
                let stage_name = args.get(3);

                println!("Stack: {}", stack_name);
                if let Some(proj) = project_name {
                    println!("Project: {}", proj);
                }
                if let Some(stage) = stage_name {
                    println!("Stage: {}", stage);
                }
                // Implement logic to execute commands based on provided names
            } else if args.len() == 5 {
                let stack_name = args.get(2);
                let project_name = args.get(3);
                let stage_name = args.get(4);

                if let Some(stack) = stack_name {
                    println!("Stack: {}", stack);
                }
                if let Some(proj) = project_name {
                    println!("Project: {}", proj);
                }
                if let Some(stage) = stage_name {
                    println!("Stage: {}", stage);
                }
                // Implement logic to execute commands based on provided names
            } else {
                println!("Project and Stage is required")
            }
            // Implement the generation logic
        },
        Commands::Version => {
            println!("--- LDOT Build Details ---");
            println!("Program Name: {}", env!("CARGO_PKG_NAME"));
            println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
            println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
            println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
            println!("Repository: {}", env!("CARGO_PKG_HOMEPAGE"));
            println!("Readme: {}", env!("CARGO_PKG_README"));
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            // Implement logic to show version
        }
        Commands::Script(_script_args) => {
            let args: Vec<String> = env::args().collect();
            println!("Executing script: {:?}", args);
            // Implement the loading logic
        }
    }
}