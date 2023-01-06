mod parameter;

pub use parameter::*;
use std::ffi::OsString;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Enable debug output
    #[structopt(short = "d", long)]
    pub debug: bool,

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
