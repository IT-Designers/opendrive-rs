/// Defines the style of the line at the outer border of a lane. The style of the center line that
/// separates left and right lanes is determined by the road mark element for the center lane.
#[derive(Debug, Clone)]
pub struct RoadMark {
    pub sway: Vec<Sway>,
    pub r#type: Option<Type>,
    pub explicit: Option<Explicit>,
}

#[derive(Debug, Clone)]
pub struct Sway {}

#[derive(Debug, Clone)]
pub struct Type {}

#[derive(Debug, Clone)]
pub struct Explicit {}
