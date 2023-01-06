use super::parameter::ParamConfigs;
use crate::util::value::SingleOrVec;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type CommandConfigs = BTreeMap<String, CommandConfig>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandConfig {
    pub base: SingleOrVec<String>,

    pub placeholder: Option<String>,

    #[serde(default)]
    pub headparams: ParamConfigs,

    #[serde(default)]
    pub middleparams: ParamConfigs,

    #[serde(default)]
    pub tailparams: ParamConfigs,
}

impl CommandConfig {
    pub fn empty_for(name: &str) -> Self {
        Self {
            base: SingleOrVec::Single(name.to_string()),
            placeholder: None,
            headparams: BTreeMap::new(),
            middleparams: BTreeMap::new(),
            tailparams: BTreeMap::new(),
        }
    }
}
