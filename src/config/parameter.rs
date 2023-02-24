use crate::util::{cstring::to_cstring, value::SingleOrVec};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, ffi::CString};
use thiserror::Error;

pub type ParamConfigs = BTreeMap<String, ParamDef>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("parameter value is required")]
    ParameterValueIsRequired,

    #[error("parameter value is not allowed")]
    ParameterValueIsNotAllowed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamDef(SingleOrVec<String>);

impl ParamDef {
    pub fn strings(&self) -> &[String] {
        self.0.as_slice()
    }

    pub fn extract_as_cstrings(
        &self,
        placeholder: &str,
        value: &Option<impl AsRef<str>>,
    ) -> Result<Vec<CString>, Error> {
        let mut extracted = Vec::with_capacity(self.0.len());

        if let Some(value) = value {
            let mut replaced = false;
            for s in self.0.as_slice() {
                let cstr = if s.contains(placeholder) {
                    replaced = true;
                    to_cstring(s.replace(placeholder, value.as_ref()))
                } else {
                    to_cstring(s.as_bytes())
                };
                extracted.push(cstr)
            }
            if !replaced {
                return Err(Error::ParameterValueIsNotAllowed);
            }
        } else {
            for s in self.0.as_slice() {
                if s.contains(placeholder) {
                    return Err(Error::ParameterValueIsRequired);
                }
                extracted.push(to_cstring(s.as_bytes()))
            }
        }
        Ok(extracted)
    }
}
