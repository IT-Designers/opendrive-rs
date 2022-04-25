use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Stores information about the material of lanes. Each element is valid until a new element is
/// defined. If multiple elements are defined, they must be listed in ascending order.
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    /// Friction coefficient
    pub friction: f64,
    /// Roughness, for example, for sound and motion systems
    pub roughness: Option<f64>,
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    pub s_offset: Length,
    /// Surface material code, depending on application
    pub surface: Option<String>,
}

impl Material {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "friction" => Some(self.friction.to_scientific_string()).as_deref(),
            "roughness" => self.roughness.map(|v| v.to_scientific_string()).as_deref(),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
            "surface" => self.surface.as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Material
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            friction: read.attribute("friction")?,
            roughness: read.attribute_opt("roughness")?,
            s_offset: read.attribute("sOffset").map(Length::new::<meter>)?,
            surface: read.attribute_opt("surface")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Material {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            friction: u.not_nan_f64()?,
            roughness: if u.arbitrary()? {
                Some(u.not_nan_f64()?)
            } else {
                None
            },
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            surface: u.arbitrary()?,
        })
    }
}
