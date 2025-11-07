use clap::{Parser, Subcommand, ValueEnum};

/// A CLI tool to quickly switch environment variables in a PowerShell session.
#[derive(Parser, Debug)] // The main CLI structure
#[command(version, about, long_about = None, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Edit a configuration file. Defaults to the local file.
    Edit(EditArgs),
    /// Show the current status of environment variables.
    Show(ShowArgs),
}

#[derive(Parser, Debug)]
pub struct EditArgs {
    /// The target configuration file to edit.
    #[arg(value_enum, default_value_t = EditTarget::Local)]
    pub target: EditTarget,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum EditTarget {
    Local,
    Global,
}

#[derive(Parser, Debug)]
pub struct ShowArgs {
    /// Reveal the actual values of the environment variables.
    #[arg(long)]
    pub reveal: bool,
}
