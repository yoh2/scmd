mod parameter;

use clap::Parser;
pub use parameter::*;
use std::ffi::OsString;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Opt {
    /// Enable debug output
    #[arg(short, long)]
    pub debug: bool,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Instead of actually performing the command, print what command would be executed.
    #[arg(long)]
    pub dry_run: bool,

    /// List the argument definitions of the specified command,
    /// or list defined commands if no commands is specified.
    #[arg(short, long)]
    pub list: bool,

    /// Passthrough unknown command
    #[arg(short, long)]
    pub passthrough: Option<bool>,

    /// Command to be run
    #[arg(required_unless_present = "list")]
    pub command: Option<String>,

    /// Command parameters
    pub parameters: Vec<Parameter>,

    /// Arguments that is passed to the program
    #[arg(last = true)]
    pub args: Vec<OsString>,
}
