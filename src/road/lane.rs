use serde_aux::field_attributes::deserialize_number_from_string;
use serde_derive::{Deserialize, Serialize};
use uom::si::f64::Length;

/// Contains a series of lane section elements that define the characteristics of the road cross
/// sections with respect to the lanes along the reference line.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lanes {
    #[serde(rename = "laneOffset", default = "Vec::new")]
    pub lane_offset: Vec<Offset>,
    #[serde(rename = "laneSection", default = "Vec::new")]
    pub lane_section: Vec<Section>,
}

/// A lane offset may be used to shift the center lane away from the road reference line.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Offset {
    /// Polynom parameter a, offset at @s (ds=0)
    pub a: f64,
    /// Polynom parameter b
    pub b: f64,
    /// Polynom parameter c
    pub c: f64,
    /// Polynom parameter d
    pub d: f64,
    /// s-coordinate of start position
    pub s: f64,
}

/// Lanes may be split into multiple lane sections. Each lane section contains a fixed number of
/// lanes. Every time the number of lanes changes, a new lane section is required. The distance
/// between two succeeding lane sections shall not be zero.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Section {
    /// s-coordinate of start position
    pub s: f64,
    /// Lane section element is valid for one side only (left, center, or right), depending on the
    /// child elements.
    #[serde(rename = "singleSide")]
    pub single_side: Option<bool>,
    pub left: Option<Left>,
    pub center: Center,
    pub right: Option<Right>,
}

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Left {
    pub lane: Vec<LeftLane>,
}

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeftLane {
    /// ID of the lane
    pub id: i64,
    #[serde(flatten)]
    pub base: Lane,
}

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Center {
    pub lane: Vec<CenterLane>,
}

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CenterLane {
    /// ID of the lane
    pub id: i64,
    #[serde(flatten)]
    pub base: Lane,
}

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Right {
    pub lane: Vec<RightLane>,
}

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RightLane {
    /// ID of the lane
    pub id: i64,
    #[serde(flatten)]
    pub base: Lane,
}

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lane {
    pub link: Option<LaneLink>,
    #[serde(flatten)]
    // todo actually unbound
    pub choice: Option<LaneChoice>,
    // #[serde(flatten, rename = "roadMark")] // todo actually unbound
    // pub material: Vec<RoadMark>, // TODO
    // #[serde(default = "Vec::new")]
    // pub material: Vec<Material>, // TODO
    // #[serde(default = "Vec::new")]
    // pub speed: Vec<Speed>, // TODO
    // #[serde(default = "Vec::new")]
    // pub access: Vec<Access>, // TODO
    // #[serde(default = "Vec::new")]
    // pub height: Vec<Height>, // TODO
    // #[serde(default = "Vec::new")]
    // pub rule: Vec<Rule>, // TODO
}

/// For links between lanes with an identical reference line, the lane predecessor and successor
/// information provide the IDs of lanes on the preceding or following lane section.
/// For links between lanes with different reference line,  the lane predecessor and successor
/// information provide the IDs of lanes on the first or last lane section of the other reference
/// line depending on the contact point of the road linkage.
/// This element may only be omitted, if lanes end at a junction or have no physical link.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LaneLink {
    #[serde(default = "Vec::new")]
    predecessor: Vec<PredecessorSuccessor>,
    #[serde(default = "Vec::new")]
    successor: Vec<PredecessorSuccessor>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PredecessorSuccessor {
    /// ID of the preceding / succeeding linked lane
    id: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LaneChoice {
    #[serde(rename = "border")]
    Border(Border),
    #[serde(rename = "width")]
    Width(Width),
}

/// Lane borders are another method to describe the width of lanes. Instead of defining the width
/// directly, lane borders describe the outer limits of a lane, independent of the parameters of
/// their inner borders. In this case, inner lanes are defined as lanes which have the same sign for
/// their ID as the lane currently defined, but with a smaller absolute value for their ID.
/// Especially when road data is derived from automatic measurements, this type of definition is
/// easier than specifying the lane width because it avoids creating many lane sections.
/// Lane width and lane border elements are mutually exclusive within the same lane group. If both
/// width and lane border elements are present for a lane section in the ASAM OpenDRIVE file, the
/// application shall use the information from the `<width>` elements.
/// In ASAM OpenDRIVE, lane borders are represented by the `<border>` element within the `<lane>`
/// element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Border {
    /// Polynom parameter a, border position at @s (ds=0)
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub a: f64,
    /// Polynom parameter b
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub b: f64,
    /// Polynom parameter c
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub c: f64,
    /// Polynom parameter d
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub d: f64,
    /// s-coordinate of start position of the `<border>` element , relative to the position of the
    /// preceding `<laneSection>` element
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "sOffset")]
    pub s_offset: Length,
}

/// The width of a lane is defined along the t-coordinate. The width of a lane may change within a
/// lane section.
/// Lane width and lane border elements are mutually exclusive within the same lane group. If both
/// width and lane border elements are present for a lane section in the ASAM OpenDRIVE file, the
/// application must use the information from the `<width>` elements.
/// In ASAM OpenDRIVE, lane width is described by the `<width>` element within the `<lane>` element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Width {
    /// Polynom parameter a, width at @s (ds=0)
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub a: f64,
    /// Polynom parameter b
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub b: f64,
    /// Polynom parameter c
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub c: f64,
    /// Polynom parameter d
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub d: f64,
    /// s-coordinate of start position of the `<width>` element, relative to the position of the
    /// preceding `<laneSection>` element
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "sOffset")]
    pub s_offset: Length,
}

/// Defines the style of the line at the outer border of a lane. The style of the center line that
/// separates left and right lanes is determined by the road mark element for the center lane.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoadMark {}
