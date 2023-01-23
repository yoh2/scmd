mod parameter;

pub use parameter::*;
use std::ffi::OsString;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Enable debug output
    #[structopt(short = "d", long)]
    pub debug: bool,

    /// Enable verbose output
    #[structopt(short = "v", long)]
    pub verbose: bool,

    /// Instead of actually performing the command, displays what command would be executed.
    #[structopt(long)]
    pub dry_run: bool,

    /// Passthrough unknown command
    #[structopt(short = "p", long)]
    pub passthrough: Option<bool>,

    /// Command to be run
    pub command: Option<String>,

    /// Command parameters
    pub parameters: Vec<Parameter>,

    /// Arguments that is passed to the program
    #[structopt(last = true)]
    pub args: Vec<OsString>,
}
