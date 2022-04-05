use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

pub mod mark;

/// Contains a series of lane section elements that define the characteristics of the road cross
/// sections with respect to the lanes along the reference line.
#[derive(Debug, Clone)]
pub struct Lanes {
    pub lane_offset: Vec<Offset>,
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Section {
    /// s-coordinate of start position
    pub s: f64,
    /// Lane section element is valid for one side only (left, center, or right), depending on the
    /// child elements.
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct LeftLane {
    /// ID of the lane
    pub id: i64,
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct CenterLane {
    /// ID of the lane
    pub id: i64,
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct RightLane {
    /// ID of the lane
    pub id: i64,
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Lane {
    pub link: Option<LaneLink>,
    pub choice: Vec<LaneChoice>,
    // pub road_mark: Vec<RoadMark>,
    // TODO
    //     // pub material: Vec<Material>, // TODO
    //     // pub speed: Vec<Speed>, // TODO
    //     // pub access: Vec<Access>, // TODO
    //     // pub height: Vec<Height>, // TODO
    //     // pub rule: Vec<Rule>, // TODO
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

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visitor(Cow::default())
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(link) = &self.link {
            visit_children!(visitor, "link" => link);
        }
        for choice in &self.choice {
            match choice {
                LaneChoice::Border(border) => visit_children!(visitor, "border" => border),
                LaneChoice::Width(width) => visit_children!(visitor, "width" => width),
            }
        }
        Ok(())
    }
}

/// For links between lanes with an identical reference line, the lane predecessor and successor
/// information provide the IDs of lanes on the preceding or following lane section.
/// For links between lanes with different reference line,  the lane predecessor and successor
/// information provide the IDs of lanes on the first or last lane section of the other reference
/// line depending on the contact point of the road linkage.
/// This element may only be omitted, if lanes end at a junction or have no physical link.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct LaneLink {
    pub predecessor: Vec<PredecessorSuccessor>,
    pub successor: Vec<PredecessorSuccessor>,
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

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(visitor)
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for predecessor in &self.predecessor {
            visit_children!(visitor, "predecessor" => predecessor);
        }
        for successor in &self.successor {
            visit_children!(visitor, "successor" => successor);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct PredecessorSuccessor {
    /// ID of the preceding / succeeding linked lane
    pub id: i64,
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

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(visitor, "id" => &self.id.to_string())
    }

    pub fn visit_children(
        &self,
        _visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum LaneChoice {
    Border(Border),
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
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    /// Polynom parameter a, border position at @s (ds=0)
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub a: f64,
    /// Polynom parameter b
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub b: f64,
    /// Polynom parameter c
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub c: f64,
    /// Polynom parameter d
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub d: f64,
    /// s-coordinate of start position of the `<border>` element , relative to the position of the
    /// preceding `<laneSection>` element
    // https://github.com/RReverser/serde-xml-rs/issues/137
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

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "a" => &self.a.to_string(),
            "b" => &self.b.to_string(),
            "c" => &self.c.to_string(),
            "d" => &self.d.to_string(),
            "sOffset" => &self.s_offset.value.to_string(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(visitor);
        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Border {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            a: u.not_nan_f64()?,
            b: u.not_nan_f64()?,
            c: u.not_nan_f64()?,
            d: u.not_nan_f64()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}

/// The width of a lane is defined along the t-coordinate. The width of a lane may change within a
/// lane section.
/// Lane width and lane border elements are mutually exclusive within the same lane group. If both
/// width and lane border elements are present for a lane section in the ASAM OpenDRIVE file, the
/// application must use the information from the `<width>` elements.
/// In ASAM OpenDRIVE, lane width is described by the `<width>` element within the `<lane>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Width {
    /// Polynom parameter a, width at @s (ds=0)
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub a: f64,
    /// Polynom parameter b
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub b: f64,
    /// Polynom parameter c
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub c: f64,
    /// Polynom parameter d
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub d: f64,
    /// s-coordinate of start position of the `<width>` element, relative to the position of the
    /// preceding `<laneSection>` element
    // https://github.com/RReverser/serde-xml-rs/issues/137
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

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "a" => &self.a.to_string(),
            "b" => &self.b.to_string(),
            "c" => &self.c.to_string(),
            "d" => &self.d.to_string(),
            "sOffset" => &self.s_offset.value.to_string(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(visitor);
        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Width {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            a: u.not_nan_f64()?,
            b: u.not_nan_f64()?,
            c: u.not_nan_f64()?,
            d: u.not_nan_f64()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}
