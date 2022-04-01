use serde_derive::{Deserialize, Serialize};

/// Defines the style of the line at the outer border of a lane. The style of the center line that
/// separates left and right lanes is determined by the road mark element for the center lane.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoadMark {
    #[serde(default = "Vec::new")]
    sway: Vec<Sway>,
    r#type: Option<Type>,
    explicit: Option<Explicit>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sway {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Type {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Explicit {}
