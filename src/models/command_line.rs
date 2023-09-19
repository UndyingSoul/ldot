use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "LDOT")]
#[command(about = "A Local Development Orchestration Tool", long_about = None, author = "UndyingSoul", version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Define the structure for command-line parameters
#[derive(Debug, Subcommand)]
enum Commands {
    // Define the fields for command-line parameters

    #[arg(value_name = "COMMIT", short = "Gets the version number of LDOT")]
    Version(),

    #[arg(value_name = "COMMAND", short = "Prints help information")]
    Help(Vec<OsString>),

    #[arg(value_name = "FILE", short = "Validates an LDOT file", long = "Validates a specified LDOT file, making sure each key is correct, and valid.", default_value = "ldot_stack.json")]
    Validate(Option<PathBuf>),

    #[arg(value_name = "FILE", short = "Generates an LDOT file", long = "Generates an LDOT file to be changed manually.", default_value = "ldot_stack.json")]
    Generate(Option<PathBuf>),
    
    #[arg(value_name = "FILE", short = "Loads an LDOT file", long = "Loads a specified LDOT file to be used anywhere. Validates before loading file.", default_value = "ldot_stack.json")]
    Load(Option<PathBuf>),

    #[arg(arg_required_else_help = true, value_name = "FILE", short = "Configures the LDOT utility")]
    Config(Option<ConfigureArgs>),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
enum ConfigureArgs {
    #[arg(short = "List registered configurations")]
    List,

    #[arg(short = "Configures default stack")]
    Default(String),

    #[arg(short = "Prints the location of the configuration file")]
    Edit,

    #[arg(short = "Regenerates configuration file")]
    Regenerate
}