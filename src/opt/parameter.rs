use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub value: Option<String>,
}

impl FromStr for Parameter {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('=') {
            Some((name, arg)) => Ok(Self {
                name: name.to_string(),
                value: Some(arg.to_string()),
            }),
            None => Ok(Self {
                name: s.to_string(),
                value: None,
            }),
        }
    }
}
