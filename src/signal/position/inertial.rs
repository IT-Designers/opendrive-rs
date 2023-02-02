use std::borrow::Cow;
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length};
use uom::si::length::meter;

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
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
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
