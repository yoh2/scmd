mod command;
mod default;
pub mod parameter;

pub use command::*;
pub use default::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub default: DefaultConfig,

    #[serde(default)]
    pub command: CommandConfigs,
}
