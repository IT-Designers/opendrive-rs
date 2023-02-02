use crate::road::unit::SpeedUnit;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Defines the maximum allowed speed on a given lane. Each element is valid in direction of the
/// increasing s-coordinate until a new element is defined.
#[derive(Debug, Clone, PartialEq)]
pub struct Speed {
    /// Maximum allowed speed. If the attribute unit is not specified, m/s is used as default.
    pub max: f64,
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    pub s_offset: Length,
    /// Unit of the attribute max
    pub unit: Option<SpeedUnit>,
}

impl Speed {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "max" => Some(self.max.to_scientific_string()).as_deref(),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
            "unit" => self.unit.as_ref().map(SpeedUnit::as_str),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Speed
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            max: read.attribute("max")?,
            s_offset: read.attribute("sOffset").map(Length::new::<meter>)?,
            unit: read.attribute_opt("unit")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Speed {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            max: u.not_nan_f64()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            unit: u.arbitrary()?,
        })
    }
}
