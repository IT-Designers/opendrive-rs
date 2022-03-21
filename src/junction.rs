use serde_derive::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ElementDir {
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = "-")]
    Minus,
}
