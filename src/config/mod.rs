mod command;
mod default;
mod envset;
pub mod parameter;

pub use command::*;
pub use default::*;
pub use envset::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub default: DefaultConfig,

    #[serde(default)]
    pub command: CommandConfigs,
}
