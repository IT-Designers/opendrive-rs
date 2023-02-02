use crate::core::additional_data::AdditionalData;
use crate::lane::road_mark::color::Color;
use crate::lane::road_mark::weight::Weight;
use explicit::Explicit;
use lane_change::LaneChange;
use r#type::Type;
use std::borrow::Cow;
use sway::Sway;
use type_simplified::TypeSimplified;
use uom::si::f64::Length;
use uom::si::length::meter;

pub mod color;
pub mod explicit;
pub mod explicit_line;
pub mod lane_change;
pub mod rule;
pub mod sway;
pub mod r#type;
pub mod type_simplified;
pub mod weight;

/// Defines the style of the line at the outer border of a lane. The style of the center line that
/// separates left and right lanes is determined by the road mark element for the center lane.
#[derive(Debug, Clone, PartialEq)]
pub struct RoadMark {
    pub sway: Vec<Sway>,
    pub r#type: Option<Type>,
    pub explicit: Option<Explicit>,
    /// Color of the road mark
    pub color: color::Color,
    /// Height of road mark above the road, i.e. thickness of the road mark
    pub height: Option<Length>,
    /// Allows a lane change in the indicated direction, taking into account that lanes are numbered
    /// in ascending order from right to left. If the attribute is missing, “both” is used as
    /// default.
    pub lane_change: Option<LaneChange>,
    /// Material of the road mark. Identifiers to be defined by the user, use "standard" as default
    /// value.
    pub material: Option<String>,
    /// s-coordinate of start position of the `<roadMark>` element, relative to the position of the
    /// preceding `<laneSection>` element
    pub s_offset: Length,
    /// Type of the road mark
    pub type_simplified: TypeSimplified,
    /// Weight of the road mark. This attribute is optional if detailed definition is given below.
    pub weight: Option<Weight>,
    /// Width of the road mark. This attribute is optional if detailed definition is given by
    /// <line> element.
    pub width: Option<Length>,
    pub additional_data: AdditionalData,
}

impl RoadMark {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "color" => Some(self.color.as_str()),
            "height" => self.height.map(|v| v.value.to_scientific_string()).as_deref(),
            "laneChange" => self.lane_change.as_ref().map(LaneChange::as_str),
            "material" => self.material.as_deref(),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
            "type" => Some(self.type_simplified.as_str()),
            "weight" => self.weight.as_ref().map(Weight::as_str),
            "width" => self.width.map(|v| v.value.to_scientific_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for sway in &self.sway {
            visit_children!(visitor, "sway" => sway);
        }

        if let Some(r#type) = &self.r#type {
            visit_children!(visitor, "type" => r#type);
        }

        if let Some(explicit) = &self.explicit {
            visit_children!(visitor, "explicit" => explicit);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for RoadMark
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut sway = Vec::new();
        let mut r#type = None;
        let mut explicit = None;
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "sway" => Sway => |v| sway.push(v),
            "type" => Type => |v| r#type = Some(v),
            "explicit" => Explicit => |v| explicit = Some(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            sway,
            r#type,
            explicit,
            color: if cfg!(feature = "workaround-sumo-roadmark-missing-color") {
                read.attribute_opt("color")?.unwrap_or(Color::Standard)
            } else {
                read.attribute("color")?
            },
            height: read.attribute_opt("height")?.map(Length::new::<meter>),
            lane_change: read.attribute_opt("laneChange")?,
            material: read.attribute_opt("material")?,
            s_offset: read.attribute("sOffset").map(Length::new::<meter>)?,
            type_simplified: read.attribute("type")?,
            weight: read.attribute_opt("weight")?,
            width: read.attribute_opt("width")?.map(Length::new::<meter>),
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for RoadMark {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            sway: u.arbitrary()?,
            r#type: u.arbitrary()?,
            explicit: u.arbitrary()?,
            color: u.arbitrary()?,
            height: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
            lane_change: u.arbitrary()?,
            material: u.arbitrary()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            type_simplified: u.arbitrary()?,
            weight: u.arbitrary()?,
            width: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
            additional_data: u.arbitrary()?,
        })
    }
}
