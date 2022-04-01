use serde_aux::field_attributes::deserialize_number_from_string;
use serde_derive::{Deserialize, Serialize};
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

pub mod mark;

/// Contains a series of lane section elements that define the characteristics of the road cross
/// sections with respect to the lanes along the reference line.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lanes {
    #[serde(rename = "laneOffset", default = "Vec::new")]
    pub lane_offset: Vec<Offset>,
    #[serde(rename = "laneSection", default = "Vec::new")]
    pub lane_section: Vec<Section>,
}

impl Lanes {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut lane_offset = Vec::new();
        let mut lane_section = Vec::new();

        find_map_parse_elem!(
            events,
            "laneOffset" => |attributes| {
                lane_offset.push(Offset::from_events(events, attributes)?);
                Ok(())
            },
            "laneSection" => |attributes| {
                lane_section.push(Section::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self {
            lane_offset,
            lane_section,
        })
    }
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

impl Offset {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            a: find_map_parse_attr!(attributes, "a", f64)?,
            b: find_map_parse_attr!(attributes, "b", f64)?,
            c: find_map_parse_attr!(attributes, "c", f64)?,
            d: find_map_parse_attr!(attributes, "d", f64)?,
            s: find_map_parse_attr!(attributes, "s", f64)?,
        })
    }
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

impl Section {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut left = None;
        let mut center = None;
        let mut right = None;

        find_map_parse_elem!(
            events,
            "left" => |attributes| {
                left = Some(Left::from_events(events, attributes)?);
                Ok(())
            },
            "center" true => |attributes| {
                center = Some(Center::from_events(events, attributes)?);
                Ok(())
            },
            "right" => |attributes| {
                right = Some(Right::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self {
            s: find_map_parse_attr!(attributes, "s", f64)?,
            single_side: find_map_parse_attr!(attributes, "singleSide", Option<bool>)?,
            left,
            center: center.expect("Marked as required"),
            right,
        })
    }
}

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Left {
    pub lane: Vec<LeftLane>,
}

impl Left {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut lane = Vec::new();

        find_map_parse_elem!(
            events,
            "lane" => |attributes| {
                lane.push(LeftLane::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self { lane })
    }
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

impl LeftLane {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        Ok(Self {
            id: find_map_parse_attr!(attributes, "id", i64)?,
            base: Lane::from_events(events, Vec::new())?,
        })
    }
}

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Center {
    pub lane: Vec<CenterLane>,
}

impl Center {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut lane = Vec::new();

        find_map_parse_elem!(
            events,
            "lane" => |attributes| {
                lane.push(CenterLane::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self { lane })
    }
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

impl CenterLane {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        Ok(Self {
            id: find_map_parse_attr!(attributes, "id", i64)?,
            base: Lane::from_events(events, Vec::new())?,
        })
    }
}

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Right {
    pub lane: Vec<RightLane>,
}

impl Right {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut lane = Vec::new();

        find_map_parse_elem!(
            events,
            "lane" => |attributes| {
                lane.push(RightLane::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self { lane })
    }
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

impl RightLane {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        Ok(Self {
            id: find_map_parse_attr!(attributes, "id", i64)?,
            base: Lane::from_events(events, Vec::new())?,
        })
    }
}

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lane {
    pub link: Option<LaneLink>,
    // #[serde(flatten, default = "Vec::new")]
    #[serde(rename = "$value", default = "Vec::new")]
    pub choice: Vec<LaneChoice>,
    // #[serde(flatten, rename = "roadMark", default = "Vec::new")]
    // pub road_mark: Vec<RoadMark>,
    // TODO
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

impl Lane {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut link = None;
        let mut choice = Vec::new();

        find_map_parse_elem!(
            events,
            "link" => |attributes| {
                link = Some(LaneLink::from_events(events, attributes)?);
                Ok(())
            },
            "border" => |attributes| {
                choice.push(LaneChoice::Border(Border::from_events(events, attributes)?));
                Ok(())
            },
            "width" => |attributes| {
                choice.push(LaneChoice::Width(Width::from_events(events, attributes)?));
                Ok(())
            },
        );

        Ok(Self { link, choice })
    }
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

impl LaneLink {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut predecessor = Vec::new();
        let mut successor = Vec::new();

        find_map_parse_elem!(
            events,
            "predecessor" => |attributes| {
                predecessor.push(PredecessorSuccessor::from_events(events, attributes)?);
                Ok(())
            },
            "successor" => |attributes| {
                successor.push(PredecessorSuccessor::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self {
            predecessor,
            successor,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PredecessorSuccessor {
    /// ID of the preceding / succeeding linked lane
    id: i64,
}

impl PredecessorSuccessor {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            id: find_map_parse_attr!(attributes, "id", i64)?,
        })
    }
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

impl Border {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            a: find_map_parse_attr!(attributes, "a", f64)?,
            b: find_map_parse_attr!(attributes, "b", f64)?,
            c: find_map_parse_attr!(attributes, "c", f64)?,
            d: find_map_parse_attr!(attributes, "d", f64)?,
            s_offset: find_map_parse_attr!(attributes, "sOffset", f64).map(Length::new::<meter>)?,
        })
    }
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

impl Width {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            a: find_map_parse_attr!(attributes, "a", f64)?,
            b: find_map_parse_attr!(attributes, "b", f64)?,
            c: find_map_parse_attr!(attributes, "c", f64)?,
            d: find_map_parse_attr!(attributes, "d", f64)?,
            s_offset: find_map_parse_attr!(attributes, "sOffset", f64).map(Length::new::<meter>)?,
        })
    }
}
