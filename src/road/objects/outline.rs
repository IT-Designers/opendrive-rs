use crate::road::lane::LaneType;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// An outline defines a series of corner points, including the height of the object relative to the
/// road reference line. The inner area of the described outline may be filled with a filling type,
/// such as grass, concrete, asphalt, or pavement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Outlines {
    pub outline: Vec<Outline>,
}

impl Outlines {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut outline = Vec::new();

        find_map_parse_elem!(
            events,
            "outline" true => |attributes| {
                outline.push(Outline::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self { outline })
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
        for outline in &self.outline {
            visit_children!(visitor, "outline" => outline);
        }
        Ok(())
    }
}

/// Defines a series of corner points, including the height of the object relative to the road
/// reference line. For areas, the points should be listed in counter-clockwise order.
/// An `<outline>` element shall be followed by one or more `<cornerRoad>` element or by one or more
/// `<cornerLocal>` element.
/// ASAM OpenDRIVE 1.4 outline definitions (without `<outlines>` parent element) shall still be
/// supported.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Outline {
    /// If true, the outline describes an area, not a linear feature
    pub closed: Option<bool>,
    /// Type used to fill the area inside the outline
    pub fill_type: Option<OutlineFillType>,
    /// ID of the outline. Must be unique within one object.
    pub id: Option<u64>,
    /// Describes the lane type of the outline
    pub lane_type: Option<LaneType>,
    /// Defines if outline is an outer outline of the object
    pub outer: Option<bool>,
    pub choice: Vec<Corner>,
}

impl Outline {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut choice = Vec::new();

        find_map_parse_elem!(
            events,
            "cornerRoad" => |attributes| {
                choice.push(Corner::Road(CornerRoad::from_events(events, attributes)?));
                Ok(())
            },
            "cornerLocal" => |attributes| {
                choice.push(Corner::Local(CornerLocal::from_events(events, attributes)?));
                Ok(())
            },
        );

        if choice.is_empty() {
            return Err(crate::parser::Error::child_missing::<Self>());
        }

        Ok(Self {
            closed: find_map_parse_attr!(attributes, "closed", Option<bool>)?,
            fill_type: find_map_parse_attr!(attributes, "fillType", Option<OutlineFillType>)?,
            id: find_map_parse_attr!(attributes, "id", Option<u64>)?,
            lane_type: find_map_parse_attr!(attributes, "laneType", Option<LaneType>)?,
            outer: find_map_parse_attr!(attributes, "outer", Option<bool>)?,
            choice,
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
            "closed" => self.closed.map(|v| v.to_string()).as_deref(),
            "fillType" => self.fill_type.as_ref().map(OutlineFillType::as_str),
            "id" => self.id.map(|v| v.to_string()).as_deref(),
            "laneType" => self.lane_type.as_ref().map(LaneType::as_str),
            "outer" => self.outer.map(|v| v.to_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for choice in &self.choice {
            match choice {
                Corner::Road(road) => visit_children!(visitor, "cornerRoad" => road),
                Corner::Local(local) => visit_children!(visitor, "cornerLocal" => local),
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Corner {
    Road(CornerRoad),
    Local(CornerLocal),
}

/// Defines a corner point on the object’s outline in road coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct CornerRoad {
    /// dz of the corner relative to road reference line
    pub dz: Length,
    /// Height of the object at this corner, along the z-axis
    pub height: Length,
    /// ID of the outline point. Must be unique within one outline
    pub id: Option<u64>,
    /// s-coordinate of the corner
    pub s: Length,
    /// t-coordinate of the corner
    pub t: Length,
}

impl CornerRoad {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            dz: find_map_parse_attr!(attributes, "dz", f64).map(Length::new::<meter>)?,
            height: find_map_parse_attr!(attributes, "height", f64).map(Length::new::<meter>)?,
            id: find_map_parse_attr!(attributes, "id", Option<u64>)?,
            s: find_map_parse_attr!(attributes, "s", f64).map(Length::new::<meter>)?,
            t: find_map_parse_attr!(attributes, "t", f64).map(Length::new::<meter>)?,
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
            "dz" => Some(self.dz.value.to_scientific_string()).as_deref(),
            "height" => Some(self.height.value.to_scientific_string()).as_deref(),
            "id" => self.id.map(|v| v.to_string()).as_deref(),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "t" => Some(self.t.value.to_scientific_string()).as_deref(),
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
impl arbitrary::Arbitrary<'_> for CornerRoad {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            dz: Length::new::<meter>(u.not_nan_f64()?),
            height: Length::new::<meter>(u.not_nan_f64()?),
            id: u.arbitrary()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
            t: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}

/// Used to describe complex forms of objects. Defines a corner point on the object outline relative
/// to the object pivot point in local u/v-coordinates. The insertion point and the orientation of
/// the object are given by the @s, @t, @zOffset and @hdg attributes of the  element.
#[derive(Debug, Clone, PartialEq)]
pub struct CornerLocal {
    /// Height of the object at this corner, along the z-axis
    pub height: Length,
    /// ID of the outline point. Shall be unique within one outline.
    pub id: Option<u64>,
    /// Local u-coordinate of the corner
    pub u: Length,
    /// Local v-coordinate of the corner
    pub v: Length,
    /// Local z-coordinate of the corner
    pub z: Length,
}

impl CornerLocal {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            height: find_map_parse_attr!(attributes, "height", f64).map(Length::new::<meter>)?,
            id: find_map_parse_attr!(attributes, "id", Option<u64>)?,
            u: find_map_parse_attr!(attributes, "u", f64).map(Length::new::<meter>)?,
            v: find_map_parse_attr!(attributes, "v", f64).map(Length::new::<meter>)?,
            z: find_map_parse_attr!(attributes, "w", f64).map(Length::new::<meter>)?,
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
            "height" => Some(self.height.value.to_scientific_string()).as_deref(),
            "id" => self.id.map(|v| v.to_string()).as_deref(),
            "u" => Some(self.u.value.to_scientific_string()).as_deref(),
            "v" => Some(self.v.value.to_scientific_string()).as_deref(),
            "z" => Some(self.z.value.to_scientific_string()).as_deref(),
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
impl arbitrary::Arbitrary<'_> for CornerLocal {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            height: Length::new::<meter>(u.not_nan_f64()?),
            id: u.arbitrary()?,
            u: Length::new::<meter>(u.not_nan_f64()?),
            v: Length::new::<meter>(u.not_nan_f64()?),
            z: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum OutlineFillType {
    Grass,
    Concrete,
    Cobble,
    Asphalt,
    Pavement,
    Gravel,
    Soil,
}

impl_from_str_as_str!(
    OutlineFillType,
    "grass" => Grass,
    "concrete" => Concrete,
    "cobble" => Cobble,
    "asphalt" => Asphalt,
    "pavement" => Pavement,
    "gravel" => Gravel,
    "soil" => Soil,
);
