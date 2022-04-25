use crate::road::lane::access::Access;
use crate::road::lane::height::Height;
use crate::road::lane::mark::RoadMark;
use crate::road::lane::material::Material;
use crate::road::lane::rule::Rule;
use crate::road::lane::speed::Speed;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use vec1::Vec1;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

pub mod access;
pub mod height;
pub mod mark;
pub mod material;
pub mod rule;
pub mod speed;

/// Contains a series of lane section elements that define the characteristics of the road cross
/// sections with respect to the lanes along the reference line.
#[derive(Debug, Clone, PartialEq)]
pub struct Lanes {
    pub lane_offset: Vec<Offset>,
    pub lane_section: Vec1<Section>,
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
            "laneSection" true => |attributes| {
                lane_section.push(Section::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self {
            lane_offset,
            lane_section: Vec1::try_from_vec(lane_section)
                .map_err(|_| crate::parser::Error::child_missing::<Self>())?,
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
        for lane_offset in &self.lane_offset {
            visit_children!(visitor, "laneOffset" => lane_offset);
        }
        for lane_section in &self.lane_section {
            visit_children!(visitor, "laneSection" => lane_section);
        }
        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Lanes {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            lane_offset: u.arbitrary()?,
            lane_section: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
        })
    }
}

/// A lane offset may be used to shift the center lane away from the road reference line.
#[derive(Debug, Clone, PartialEq)]
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

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "a" => &self.a.to_scientific_string(),
            "b" => &self.b.to_scientific_string(),
            "c" => &self.c.to_scientific_string(),
            "d" => &self.d.to_scientific_string(),
            "s" => &self.s.to_scientific_string(),
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
impl arbitrary::Arbitrary<'_> for Offset {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            a: u.not_nan_f64()?,
            b: u.not_nan_f64()?,
            c: u.not_nan_f64()?,
            d: u.not_nan_f64()?,
            s: u.not_nan_f64()?,
        })
    }
}

/// Lanes may be split into multiple lane sections. Each lane section contains a fixed number of
/// lanes. Every time the number of lanes changes, a new lane section is required. The distance
/// between two succeeding lane sections shall not be zero.
#[derive(Debug, Clone, PartialEq)]
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

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "s" => Some(self.s.to_scientific_string()).as_deref(),
            "singleSide" => self.single_side.map(|b| b.to_string()).as_deref()
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(left) = &self.left {
            visit_children!(visitor, "left" => left);
        }

        visit_children!(visitor, "center" => self.center);

        if let Some(right) = &self.right {
            visit_children!(visitor, "right" => right);
        }

        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Section {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            s: u.not_nan_f64()?,
            single_side: u.arbitrary()?,
            left: u.arbitrary()?,
            center: u.arbitrary()?,
            right: u.arbitrary()?,
        })
    }
}

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Left {
    pub lane: Vec1<LeftLane>,
}

impl Left {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut lane = Vec::new();

        find_map_parse_elem!(
            events,
            "lane" true => |attributes| {
                lane.push(LeftLane::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self {
            lane: Vec1::try_from_vec(lane)
                .map_err(|_| crate::parser::Error::child_missing::<Self>())?,
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
        for lane in &self.lane {
            visit_children!(visitor, "lane" => lane);
        }
        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Left {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            lane: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
        })
    }
}

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
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
            base: Lane::from_events(events, attributes)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        self.base.visit_attributes(|attributes| {
            let mut attributes = attributes.to_vec();
            let value = self.id.to_string();
            attributes.push(xml::attribute::Attribute::new(
                xml::name::Name::local("id"),
                &value,
            ));
            visitor(Cow::Owned(attributes))
        })
    }

    pub fn visit_children(
        &self,
        visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        self.base.visit_children(visitor)
    }
}

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Center {
    pub lane: Vec1<CenterLane>,
}

impl Center {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut lane = Vec::new();

        find_map_parse_elem!(
            events,
            "lane" true => |attributes| {
                lane.push(CenterLane::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self {
            lane: Vec1::try_from_vec(lane)
                .map_err(|_| crate::parser::Error::child_missing::<Self>())?,
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
        for lane in &self.lane {
            visit_children!(visitor, "lane" => lane);
        }
        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Center {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            lane: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
        })
    }
}

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
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
            base: Lane::from_events(events, attributes)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        self.base.visit_attributes(|attributes| {
            let mut attributes = attributes.to_vec();
            let value = self.id.to_string();
            attributes.push(xml::attribute::Attribute::new(
                xml::name::Name::local("id"),
                &value,
            ));
            visitor(Cow::Owned(attributes))
        })
    }

    pub fn visit_children(
        &self,
        visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        self.base.visit_children(visitor)
    }
}

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Right {
    pub lane: Vec1<RightLane>,
}

impl Right {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut lane = Vec::new();

        find_map_parse_elem!(
            events,
            "lane" true => |attributes| {
                lane.push(RightLane::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self {
            lane: Vec1::try_from_vec(lane)
                .map_err(|_| crate::parser::Error::child_missing::<Self>())?,
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
        for lane in &self.lane {
            visit_children!(visitor, "lane" => lane);
        }
        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Right {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            lane: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
        })
    }
}

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
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
            base: Lane::from_events(events, attributes)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        self.base.visit_attributes(|attributes| {
            let mut attributes = attributes.to_vec();
            let value = self.id.to_string();
            attributes.push(xml::attribute::Attribute::new(
                xml::name::Name::local("id"),
                &value,
            ));
            visitor(Cow::Owned(attributes))
        })
    }

    pub fn visit_children(
        &self,
        visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        self.base.visit_children(visitor)
    }
}

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, PartialEq)]
pub struct Lane {
    pub link: Option<LaneLink>,
    pub choice: Vec1<LaneChoice>,
    pub road_mark: Vec<RoadMark>,
    pub material: Vec<Material>,
    pub speed: Vec<Speed>,
    pub access: Vec<Access>,
    pub height: Vec<Height>,
    pub rule: Vec<Rule>,
    /// - `true` = keep lane on level, that is, do not apply superelevation;
    /// - `false` = apply superelevation to this lane (default, also used if attribute level is missing)
    pub level: Option<bool>,
    /// Type of the lane
    pub r#type: LaneType,
}

impl Lane {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut link = None;
        let mut choice = Vec::new();
        let mut road_mark = Vec::new();
        let mut material = Vec::new();
        let mut speed = Vec::new();
        let mut access = Vec::new();
        let mut height = Vec::new();
        let mut rule = Vec::new();

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
            "roadMark" => |attributes| {
                road_mark.push(RoadMark::from_events(events, attributes)?);
                Ok(())
            },
            "material" => |attributes| {
                material.push(Material::from_events(events, attributes)?);
                Ok(())
            },
            "speed" => |attributes| {
                speed.push(Speed::from_events(events, attributes)?);
                Ok(())
            },
            "access" => |attributes| {
                access.push(Access::from_events(events, attributes)?);
                Ok(())
            },
            "height" => |attributes| {
                height.push(Height::from_events(events, attributes)?);
                Ok(())
            },
            "rule" => |attributes| {
                rule.push(Rule::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self {
            link,
            choice: Vec1::try_from_vec(choice)
                .map_err(|_| crate::parser::Error::child_missing::<Self>())?,
            road_mark,
            material,
            speed,
            access,
            height,
            rule,
            level: find_map_parse_attr!(attributes, "level", Option<bool>)?,
            r#type: find_map_parse_attr!(attributes, "type", LaneType)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "level" => self.level.map(|v| v.to_string()).as_deref(),
            "type" => Some(self.r#type.as_str()),
        )
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

        for road_mark in &self.road_mark {
            visit_children!(visitor, "roadMark" => road_mark);
        }

        for material in &self.material {
            visit_children!(visitor, "material" => material);
        }

        for speed in &self.speed {
            visit_children!(visitor, "speed" => speed);
        }

        for access in &self.access {
            visit_children!(visitor, "access" => access);
        }

        for height in &self.height {
            visit_children!(visitor, "height" => height);
        }

        for rule in &self.rule {
            visit_children!(visitor, "rule" => rule);
        }

        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Lane {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            link: u.arbitrary()?,
            choice: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            road_mark: u.arbitrary()?,
            material: u.arbitrary()?,
            speed: u.arbitrary()?,
            access: u.arbitrary()?,
            height: u.arbitrary()?,
            rule: u.arbitrary()?,
            level: u.arbitrary()?,
            r#type: u.arbitrary()?,
        })
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
            "a" => &self.a.to_scientific_string(),
            "b" => &self.b.to_scientific_string(),
            "c" => &self.c.to_scientific_string(),
            "d" => &self.d.to_scientific_string(),
            "sOffset" => &self.s_offset.value.to_scientific_string(),
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
            "a" => &self.a.to_scientific_string(),
            "b" => &self.b.to_scientific_string(),
            "c" => &self.c.to_scientific_string(),
            "d" => &self.d.to_scientific_string(),
            "sOffset" => &self.s_offset.value.to_scientific_string(),
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

/// The lane type is defined per lane. A lane type defines the main purpose of a lane and its
/// corresponding traffic rules.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum LaneType {
    /// Describes a soft shoulder  at the edge of the roa
    Shoulder,
    /// Describes a hard border at the edge of the road. has the same height as the drivable lane
    Border,
    /// “normal” drivable road, which is not one of the other type
    Driving,
    /// Hard shoulder on motorways for emergency stop
    Stop,
    /// "Invisible" lane. This lane is on the most ouside of the road. Its only purpose is for simulation, that there is still opendrive present in case the (human) driver leaves the road.
    None,
    /// Lane on which cars should not drive, but have the same height as the drivable lanes. Typically they are separated with lines and often there are additional striped lines on them.
    Restricted,
    /// Lane with parking space
    Parking,
    /// Lane between driving lanes in oposite directions. Typically used in towns on large roads, to separate the traffic
    Median,
    /// Lane reserved for Cyclists
    Biking,
    /// Lane on which pedestrians can walk savel
    Sidewalk,
    /// Lane "curb" is used for curbstones. These have a different height compared to the drivable lanes
    Curb,
    /// Lane Type „exit“ is used for the sections which is parallel to the main road (meaning deceleration lanes)
    Exit,
    /// Lane Type „entry“ is used for the sections which is parallel to the main road (meaning acceleration lane
    Entry,
    /// A ramp leading to a motorway from rural/urban roads is an „onRamp“.
    OnRamp,
    /// A ramp leading away from a motorway and onto rural/urban roads is an „offRamp”.
    OffRamp,
    /// A ramp connecting two motorways is a „connectingRamp“ (e.g. motorway junction
    ConnectingRamp,
    /// this lane type has two use cases: a) only driving lane on a narrow road which may be used in both directions; b) continuous two-way left turn lane on multi-lane roads – US road network
    Bidirectional,
    Special1,
    Special2,
    Special3,
    RoadWorks,
    Tram,
    Rail,
    Bus,
    Taxi,
    HOV,
}

impl_from_str_as_str!(
    LaneType,
    "shoulder" => Shoulder,
    "border" => Border,
    "driving" => Driving,
    "stop" => Stop,
    "none" => None,
    "restricted" => Restricted,
    "parking" => Parking,
    "median" => Median,
    "biking" => Biking,
    "sidewalk" => Sidewalk,
    "curb" => Curb,
    "exit" => Exit,
    "entry" => Entry,
    "onRamp" => OnRamp,
    "offRamp" => OffRamp,
    "connectingRamp" => ConnectingRamp,
    "bidirectional" => Bidirectional,
    "special1" => Special1,
    "special2" => Special2,
    "special3" => Special3,
    "roadWorks" => RoadWorks,
    "tram" => Tram,
    "rail" => Rail,
    "bus" => Bus,
    "taxi" => Taxi,
    "HOV" => HOV,
);
