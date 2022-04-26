use std::borrow::Cow;
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length};
use uom::si::length::meter;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum PositionChoice {
    Inertial(PositionInertial),
    Road(PositionRoad),
}

/// Describes the reference point of the physical position in inertial coordinates in cases where it
/// deviates from the logical position. Defines the inertial position.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionInertial {
    /// Heading of the signal, relative to the inertial system
    pub hdg: Angle,
    /// Pitch angle of the signal after applying heading, relative to the inertial system
    /// (x’y’-plane)
    pub pitch: Option<Angle>,
    /// Roll angle of the signal after applying heading and pitch, relative to the inertial system
    /// (x’’y’’-plane)
    pub roll: Option<Angle>,
    /// x-coordinate
    pub x: Length,
    /// y-coordinate
    pub y: Length,
    /// z-coordinate
    pub z: Length,
}

impl PositionInertial {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "hdg" => Some(self.hdg.value.to_scientific_string()).as_deref(),
            "pitch" => self.pitch.map(|v| v.value.to_scientific_string()).as_deref(),
            "roll" => self.roll.map(|v| v.value.to_scientific_string()).as_deref(),
            "x" => Some(self.x.value.to_scientific_string()).as_deref(),
            "y" => Some(self.y.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for PositionInertial
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            hdg: Angle::new::<radian>(read.attribute("hdg")?),
            pitch: read.attribute_opt("pitch")?.map(Angle::new::<radian>),
            roll: read.attribute_opt("roll")?.map(Angle::new::<radian>),
            x: Length::new::<meter>(read.attribute("x")?),
            y: Length::new::<meter>(read.attribute("y")?),
            z: Length::new::<meter>(read.attribute("z")?),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for PositionInertial {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            hdg: Angle::new::<radian>(u.not_nan_f64()?),
            pitch: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Angle::new::<radian>),
            roll: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Angle::new::<radian>),
            x: Length::new::<meter>(u.not_nan_f64()?),
            y: Length::new::<meter>(u.not_nan_f64()?),
            z: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}

/// Describes the reference point of the physical position road coordinates in cases where it
/// deviates from the logical position. Defines the position on the road.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionRoad {
    /// Heading offset of the signal (relative to @orientation)
    pub h_offset: Angle,
    /// Pitch angle of the signal after applying hOffset, relative to the inertial system
    /// (x’y’-plane)
    pub pitch: Option<Angle>,
    /// Unique ID of the referenced road
    pub road_id: String,
    /// Roll angle of the signal after applying hOffset and pitch, relative to the inertial system
    /// (x’’y’’-plane)
    pub roll: Option<Angle>,
    /// s-coordinate
    pub s: Length,
    /// t-coordinate
    pub t: Length,
    /// z offset from road level to bottom edge of the signal
    pub z_offset: Length,
}

impl PositionRoad {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "hOffset" => Some(self.h_offset.value.to_scientific_string()).as_deref(),
            "pitch" => self.pitch.map(|v| v.value.to_scientific_string()).as_deref(),
            "roadId" => Some(self.road_id.as_str()),
            "roll" => self.roll.map(|v| v.value.to_scientific_string()).as_deref(),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "t" => Some(self.t.value.to_scientific_string()).as_deref(),
            "zOffset" => Some(self.z_offset.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for PositionRoad
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            h_offset: Angle::new::<radian>(read.attribute("hOffset")?),
            pitch: read.attribute_opt("pitch")?.map(Angle::new::<radian>),
            road_id: read.attribute("roadId")?,
            roll: read.attribute_opt("roll")?.map(Angle::new::<radian>),
            s: Length::new::<meter>(read.attribute("s")?),
            t: Length::new::<meter>(read.attribute("t")?),
            z_offset: Length::new::<meter>(read.attribute("zOffset")?),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for PositionRoad {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            h_offset: Angle::new::<radian>(u.not_nan_f64()?),
            pitch: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Angle::new::<radian>),
            road_id: u.arbitrary()?,
            roll: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Angle::new::<radian>),
            s: Length::new::<meter>(u.not_nan_f64()?),
            t: Length::new::<meter>(u.not_nan_f64()?),
            z_offset: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}
