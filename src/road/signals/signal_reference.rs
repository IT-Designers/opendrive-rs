use crate::road::objects::validity::LaneValidity;
use crate::road::objects::Orientation;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Refers to the same, that is, identical signal from multiple roads. The referenced signals
/// require a unique ID. The `<signalReference>` element consists of a main element and an optional
/// lane validity element.
#[derive(Debug, Clone, PartialEq)]
pub struct SignalReference {
    pub validity: Vec<LaneValidity>,
    /// Unique ID of the referenced signal within the database
    pub id: String,
    /// - "+" = valid in positive s-direction
    /// - "-" = valid in negative s-direction
    /// - "none" = valid in both directions
    pub orientation: Orientation,
    /// s-coordinate
    pub s: Length,
    /// t-coordinate
    pub t: Length,
}

impl SignalReference {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "id" => self.id.as_str(),
            "orientation" => self.orientation.as_str(),
            "s" => self.s.value.to_scientific_string().as_str(),
            "t" => self.t.value.to_scientific_string().as_str(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for validity in &self.validity {
            visit_children!(visitor, "validity" => validity);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for SignalReference
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut validity = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "validity" => LaneValidity => |v| validity.push(v),
        );

        Ok(Self {
            validity,
            id: read.attribute("id")?,
            orientation: read.attribute("orientation")?,
            s: Length::new::<meter>(read.attribute("s")?),
            t: Length::new::<meter>(read.attribute("t")?),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for SignalReference {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            validity: u.arbitrary()?,
            id: u.arbitrary()?,
            orientation: u.arbitrary()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
            t: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}
