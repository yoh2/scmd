use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultConfig {
    #[serde(default)]
    pub passthrough_unknown_command: bool,

    #[serde(default)]
    pub placeholder: String,
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            passthrough_unknown_command: false,
            placeholder: "{}".to_string(),
        }
    }
}
