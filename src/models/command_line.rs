use std::ffi::OsString;
use std::path::PathBuf;

use clap::{command, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "LDOT", about = "A Local Development Orchestration Tool", long_about = None, author = "UndyingSoul", version = env!("CARGO_PKG_VERSION"), disable_help_flag = true, disable_version_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// Define the structure for command-line parameters
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Gets the version number of LDOT")]
    Version,

    #[command(
        about = "Validates an LDOT file",
        long_about = "Validates a specified LDOT file, making sure each key is correct, and valid."
    )]
    Validate(ValidateArgs),

    #[command(
        about = "Generates an LDOT file",
        long_about = "Generates an LDOT file to be changed manually."
    )]
    Generate,

    #[command(
        about = "Loads an LDOT file",
        long_about = "Loads a specified LDOT file to be used anywhere. Validates before loading file."
    )]
    Load(LoadArgs),

    #[command(
        about = "Unloads an LDOT file",
        long_about = "Unloads a specified LDOT file, so it can no longer be used with LDOT."
    )]
    Unload(UnloadArgs),

    #[command(about = "Configures the LDOT utility")]
    Config(ConfigSubcommand),

    #[command(about = "Executes an LDOT stack command")]
    Execute(ExecuteArgs),

    #[command(about = "Executes an LDOT script command")]
    Script(ScriptArgs),
}

#[derive(Debug, Args)]
pub struct ConfigSubcommand {
    #[clap(subcommand)]
    pub subcommand: ConfigArgs,
}

#[derive(Debug, Args)]
pub struct ValidateArgs {
    #[arg(value_name = "FILE", default_value = "ldot_stack.json")]
    pub file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct LoadArgs {
    #[arg(value_name = "FILE", default_value = "ldot_stack.json")]
    pub file: Option<PathBuf>,
}
#[derive(Debug, Args)]
pub struct UnloadArgs {
    #[arg(value_name = "FILE", default_value = "ldot_stack.json")]
    pub file: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum ConfigArgs {
    #[command(about = "Lists LDOT configurations")]
    List(ListArgs),

    #[command(about = "Sets default LDOT stack name")]
    Default(DefaultArgs),

    // #[command(about = "Prints location of configuration file")]
    // Edit(EditArgs),

    #[command(about = "Regenerates LDOT config file", alias = "reset")]
    Regenerate(RegenerateArgs),
}

#[derive(Debug, Args)]
pub struct ListArgs;

#[derive(Debug, Args)]
pub struct DefaultArgs {
    #[arg(help = "Stack name")]
    pub stack: Option<OsString>,
}

// #[derive(Debug, Args)]
// pub struct EditArgs;

#[derive(Debug, Args)]
pub struct RegenerateArgs;

#[derive(Debug, Args)]
pub struct ExecuteArgs {
    args: Vec<OsString>, // Define fields for executing here
}

#[derive(Debug, Args)]
pub struct ScriptArgs {
    args: Vec<OsString>, // Define fields for executing here
}
