extern crate clap;
mod models {
    pub mod command_line; // This should match your file structure.
    pub mod ldot_config_json;
    pub mod stack_config_json;
}
mod utils {
    pub mod configuration_util;
    mod generic_utils;
    pub mod ldot_stack_util;
}

use std::{env, path::PathBuf};

use clap::Parser;
use colored::Colorize;
use models::{
    command_line::{Cli, Commands, ConfigArgs},
    ldot_config_json::Configuration,
};

use crate::utils::{configuration_util, ldot_stack_util};

fn main() {
    let cli = Cli::parse();
    let mut configuration: Configuration;

    match configuration_util::get_configuration() {
        Ok(config) => {
            configuration = config;
        }
        Err(err) => {
            println!("{}", "--- Startup ---".blue());
            eprintln!("Could not fetch LDOT configuration: {}", err);
            configuration = configuration_util::regenerate_configuration().unwrap();
            println!("Regenerated configuration");
        }
    }

    match configuration_util::validate_stack_names(configuration.default_stack.as_str()) {
        Ok(_) => {
            // intentionally do nothing
        }
        Err(err) => {
            eprintln!("{}\nResetting it.", err);
            configuration.default_stack = String::from("");
            let _ = configuration_util::write_configuration(&configuration);
        }
    }

    // Handle subcommands and arguments
    match cli.command {
        Commands::Validate(args) => {
            let filename: PathBuf = args.file.unwrap().to_path_buf();
            println!("{}", "--- Validating Configuration File ---".blue());
            println!("Filename: {}", filename.to_string_lossy());

            // Validation logic
            match ldot_stack_util::validate_ldot_stack(filename.as_path().display().to_string()) {
                Ok(_) => {
                    println!("Validation succeeded.");
                }
                Err(err) => {
                    eprintln!("Validation failed: {}", err);
                }
            }
        }
        Commands::Generate => {
            println!("{}", "--- Generating Configuration File ---".blue());
            let _ = ldot_stack_util::generate_ldot_stack();
        }
        Commands::Load(args) => {
            let filename: PathBuf = args.file.unwrap().to_path_buf();
            println!("{}", "--- Loading Configuration File ---".blue());
            println!("Filename: {}", filename.to_string_lossy());
            _ = ldot_stack_util::load_ldot_stack(filename.as_path().display().to_string());
            // Implement the loading logic
        }
        Commands::Unload(args) => {
            let filename: PathBuf = args.file.unwrap().to_path_buf();
            println!("{}", "--- Unloading Configuration File ---".blue());
            println!("Filename: {}", filename.to_string_lossy());
            _ = ldot_stack_util::unload_ldot_stack(filename.as_path().display().to_string());
            // Implement the loading logic
        }
        Commands::Config(config_command) => match config_command.subcommand {
            ConfigArgs::List(_) => {
                println!("{}", "--- Configuring LDOT ---".blue());
                println!("Listing Config");
                let _ = configuration_util::list_configurations();
            }
            ConfigArgs::Default(default_args) => {
                println!("{}", "--- Configuring LDOT ---".blue());
                if let Some(stack) = default_args.stack {
                    match configuration_util::set_default_stack(&stack.to_string_lossy()) {
                        Ok(_) => {
                            println!("Set default stack to: {}", stack.to_string_lossy());
                        }
                        Err(err) => {
                            eprintln!("{}", err)
                        }
                    }
                } else {
                    println!(
                        "Default stack is: {}\nConfiguration file remains unchanged.",
                        configuration.default_stack
                    );
                }
            }
            ConfigArgs::Regenerate(_) => {
                println!("{}", "--- Configuring LDOT ---".blue());
                println!("Regenerating the LDOT config file");
                let _ = configuration_util::regenerate_configuration();
            }
        },
        Commands::Execute(_execute_args) => {
            let args: Vec<String> = env::args().collect();
            println!("{}", "--- Executing Stack Commands ---".blue());
            // Handle the case where stack/project/stage names are provided
            if args.len() == 4 && configuration.default_stack != "" {
                let project_name = args.get(2);
                let stage_name = args.get(3);

                println!("Stack: {}", configuration.default_stack);
                if let Some(proj) = project_name {
                    println!("Project: {}", proj);
                }
                if let Some(stage) = stage_name {
                    println!("Stage: {}", stage);
                }

                match ldot_stack_util::execute_stack_stage_script(
                    configuration.default_stack,
                    project_name.unwrap().to_string(),
                    stage_name.unwrap().to_string(),
                ) {
                    Ok(cmds) => {
                        println!("Commands Executed:");
                        for cmd in &cmds {
                            println!("    {}", cmd);
                        }
                    }
                    Err(err) => {
                        println!("Commands Executed (Errors Occured):");
                        for err in &err {
                            println!("    {}", err);
                        }
                    }
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

                match ldot_stack_util::execute_stack_stage_script(
                    stack_name.unwrap().to_string(),
                    project_name.unwrap().to_string(),
                    stage_name.unwrap().to_string(),
                ) {
                    Ok(cmds) => {
                        println!("\nCommands Executed:");
                        for cmd in &cmds {
                            println!("    {}", cmd);
                        }
                    }
                    Err(err) => {
                        println!("\nCommands Executed (Errors Occured):");
                        for err in &err {
                            println!("    {}", err);
                        }
                    }
                }
                // Implement logic to execute commands based on provided names
            } else {
                println!("{}Project and Stage is required", if configuration.default_stack == "" {"Stack, "} else {""})
            }
            // Implement the generation logic
        }
        Commands::Version => {
            println!("{}", "--- LDOT Build Details ---".blue());
            println!("Program Name: {}", env!("CARGO_PKG_NAME"));
            println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
            println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
            println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
            println!("Repository: {}", env!("CARGO_PKG_HOMEPAGE"));
            //println!("Readme: {}", env!("CARGO_PKG_README"));
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Script(_script_args) => {
            println!("{}", "--- Executing Script ---".blue());
            let args: Vec<String> = env::args().collect();

            if args.len() == 3 && configuration.default_stack != "" {
                let script_name = args.get(2);

                if let Some(script) = script_name {
                    println!("Script: {}", script);
                }

                match ldot_stack_util::execute_stack_script(
                    configuration.default_stack,
                    script_name.unwrap().to_string()
                ) {
                    Ok(cmds) => {
                        println!("\nCommands Executed:");
                        for cmd in &cmds {
                            println!("    {}", cmd);
                        }
                    }
                    Err(err) => {
                        println!("\nCommands Executed (Errors Occured):");
                        for err in &err {
                            println!("    {}", err);
                        }
                    }
                }
            } else if args.len() == 4 {
                let stack_name = args.get(2);
                let script_name = args.get(3);

                if let Some(script) = script_name {
                    println!("Script: {}", script);
                }
                if let Some(stack) = stack_name {
                    println!("Stack: {}", stack);
                }

                match ldot_stack_util::execute_stack_script(
                    stack_name.unwrap().to_string(),
                    script_name.unwrap().to_string()
                ) {
                    Ok(cmds) => {
                        println!("\nCommands Executed:");
                        for cmd in &cmds {
                            println!("    {}", cmd);
                        }
                    }
                    Err(err) => {
                        println!("\nCommands Executed (Errors Occured):");
                        for err in &err {
                            println!("    {}", err);
                        }
                    }
                }
            } else {
                println!("Script {} name is required", if configuration.default_stack == "" {"and Stack"} else {""})
            }
        }
    }
}
