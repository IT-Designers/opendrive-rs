use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ConnectionType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "verticval")]
    Vertical,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ContactPoint {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
}

impl FromStr for ContactPoint {
    type Err = crate::parser::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("start") => Ok(Self::Start),
            _ if s.eq_ignore_ascii_case("end") => Ok(Self::End),
            _ => Err(crate::parser::Error::invalid_value_for::<Self, _>(s)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ElementDir {
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = "-")]
    Minus,
}

impl FromStr for ElementDir {
    type Err = crate::parser::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            _ => Err(crate::parser::Error::invalid_value_for::<Self, _>(s)),
        }
    }
}
