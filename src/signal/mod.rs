use crate::core::additional_data::AdditionalData;
use crate::object::lane_validity::LaneValidity;
use crate::object::orientation::Orientation;
use crate::road::country_code::CountryCode;
use crate::road::unit::Unit;
use crate::signal::dependency::Dependency;
use crate::signal::position::inertial::PositionInertial;
use crate::signal::position::road::PositionRoad;
use crate::signal::position::Position;
use crate::signal::reference::Reference;
use std::borrow::Cow;
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length};
use uom::si::length::meter;

pub mod control;
pub mod controller;
pub mod dependency;
pub mod position;
pub mod reference;
pub mod signal_reference;
pub mod signals;

/// Used to provide information about signals along a road. Consists of a main element and an
/// optional lane validity element. The element for a signal is `<signal>`.
#[derive(Debug, Clone, PartialEq)]
pub struct Signal {
    pub validity: Vec<LaneValidity>,
    pub dependency: Vec<Dependency>,
    pub reference: Vec<Reference>,
    pub choice: Option<Position>,
    /// Country code of the road, see ISO 3166-1, alpha-2 codes.
    pub country: Option<CountryCode>,
    /// Defines the year of the applied traffic rules
    pub country_revision: Option<String>,
    /// Indicates whether the signal is dynamic or static. Example: traffic light is dynamic
    pub dynamic: bool,
    /// Height of the signal, measured from bottom edge of the signal
    pub height: Option<Length>,
    /// Heading offset of the signal (relative to @orientation, if orientation is equal to “+” or “-“)
    /// Heading offset of the signal (relative to reference line, if orientation is equal to “none” )
    pub h_offset: Option<Length>,
    /// Unique ID of the signal within the OpenDRIVE file
    pub id: String,
    /// Name of the signal. May be chosen freely.
    pub name: Option<String>,
    /// - "+" = valid in positive s- direction
    /// - "-" = valid in negative s- direction
    /// - "none" = valid in both directions
    pub orientation: Orientation,
    /// Pitch angle of the signal, relative to the inertial system (xy-plane)
    pub pitch: Option<Angle>,
    /// Roll angle of the signal after applying pitch, relative to the inertial system
    /// (x’’y’’-plane)
    pub roll: Option<Angle>,
    /// s-coordinate
    pub s: Length,
    /// Subtype identifier according to country code or "-1" / "none"
    pub subtype: String,
    /// t-coordinate
    pub t: Length,
    /// Additional text associated with the signal, for example, text on city limit
    /// "City\nBadAibling"
    pub text: Option<String>,
    /// Type identifier according to country code or "-1" / "none". See extra document.
    pub r#type: String,
    /// Unit of @value
    pub unit: Option<Unit>,
    /// Value of the signal, if value is given, unit is mandatory
    pub value: Option<f64>,
    /// Width of the signal
    pub width: Option<Length>,
    /// z offset from the road to bottom edge of the signal. This represents the vertical clearance
    /// of the object. Relative to the reference line.
    pub z_offset: Length,
    pub additional_data: AdditionalData,
}

impl Signal {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "country" => self.country.as_ref().map(CountryCode::as_str),
            "countryRevision" => self.country_revision.as_deref(),
            "dynamic" => Some(if self.dynamic { "yes" } else { "no" }),
            "height" => self.height.map(|v| v.value.to_scientific_string()).as_deref(),
            "hOffset" => self.h_offset.map(|v| v.value.to_scientific_string()).as_deref(),
            "id" => Some(self.id.as_str()),
            "name" => self.name.as_deref(),
            "orientation" => Some(self.orientation.as_str()),
            "pitch" => self.pitch.map(|v| v.value.to_scientific_string()).as_deref(),
            "roll" => self.roll.map(|v| v.value.to_scientific_string()).as_deref(),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "subtype" => Some(self.subtype.as_str()),
            "t" => Some(self.t.value.to_scientific_string()).as_deref(),
            "text" => self.text.as_deref(),
            "type" => Some(self.r#type.as_str()),
            "unit" => self.unit.as_ref().map(Unit::as_str),
            "value" => self.value.map(|v| v.to_scientific_string()).as_deref(),
            "width" => self.width.map(|v| v.value.to_scientific_string()).as_deref(),
            "zOffset" => Some(self.z_offset.value.to_scientific_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for validity in &self.validity {
            visit_children!(visitor, "validity" => validity);
        }

        for dependency in &self.dependency {
            visit_children!(visitor, "dependency" => dependency);
        }

        for reference in &self.reference {
            visit_children!(visitor, "reference" => reference);
        }

        match &self.choice {
            Some(Position::Inertial(v)) => visit_children!(visitor, "positionInertial" => v),
            Some(Position::Road(v)) => visit_children!(visitor, "positionRoad" => v),
            None => {}
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Signal
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut validity = Vec::new();
        let mut dependency = Vec::new();
        let mut reference = Vec::new();
        let mut choice = None;
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "validity" => LaneValidity => |v| validity.push(v),
            "dependency" => Dependency => |v| dependency.push(v),
            "reference" => Reference => |v| reference.push(v),
            "positionInertial" => PositionInertial => |v| choice = Some(Position::Inertial(v)),
            "positionRoad" => PositionRoad => |v| choice = Some(Position::Road(v)),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            validity,
            dependency,
            reference,
            choice,
            country: read.attribute_opt("country")?,
            country_revision: read.attribute_opt("countryRevision")?,
            dynamic: read
                .attribute::<String>("dynamic")
                .map(|v| v.eq_ignore_ascii_case("yes"))?,
            height: read.attribute_opt("height")?.map(Length::new::<meter>),
            h_offset: read.attribute_opt("hOffset")?.map(Length::new::<meter>),
            id: read.attribute("id")?,
            name: read.attribute_opt("name")?,
            orientation: read.attribute("orientation")?,
            pitch: read.attribute_opt("pitch")?.map(Angle::new::<radian>),
            roll: read.attribute_opt("roll")?.map(Angle::new::<radian>),
            s: read.attribute("s").map(Length::new::<meter>)?,
            subtype: read.attribute("subtype")?,
            t: read.attribute("t").map(Length::new::<meter>)?,
            text: read.attribute_opt("text")?,
            r#type: read.attribute("type")?,
            unit: read.attribute_opt("unit")?,
            value: read.attribute_opt("value")?,
            width: read.attribute_opt("width")?.map(Length::new::<meter>),
            z_offset: read.attribute("zOffset").map(Length::new::<meter>)?,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Signal {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            validity: u.arbitrary()?,
            dependency: u.arbitrary()?,
            reference: u.arbitrary()?,
            choice: u.arbitrary()?,
            country: u.arbitrary()?,
            country_revision: u.arbitrary()?,
            dynamic: u.arbitrary()?,
            height: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64().map(Length::new::<meter>))
                .transpose()?,
            h_offset: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64().map(Length::new::<meter>))
                .transpose()?,
            id: u.arbitrary()?,
            name: u.arbitrary()?,
            orientation: u.arbitrary()?,
            pitch: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64().map(Angle::new::<radian>))
                .transpose()?,
            roll: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64().map(Angle::new::<radian>))
                .transpose()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
            subtype: u.arbitrary()?,
            t: Length::new::<meter>(u.not_nan_f64()?),
            text: u.arbitrary()?,
            r#type: u.arbitrary()?,
            unit: u.arbitrary()?,
            value: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?,
            width: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64().map(Length::new::<meter>))
                .transpose()?,
            z_offset: Length::new::<meter>(u.not_nan_f64()?),
            additional_data: u.arbitrary()?,
        })
    }
}
