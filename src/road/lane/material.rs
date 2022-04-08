use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Stores information about the material of lanes. Each element is valid until a new element is
/// defined. If multiple elements are defined, they must be listed in ascending order.
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    friction: f64,
    roughness: Option<f64>,
    s_offset: Length,
    surface: String,
}

impl Material {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            friction: find_map_parse_attr!(attributes, "friction", f64)?,
            roughness: find_map_parse_attr!(attributes, "roughness", Option<f64>)?,
            s_offset: find_map_parse_attr!(attributes, "sOffset", f64).map(Length::new::<meter>)?,
            surface: find_map_parse_attr!(attributes, "surface", String)?,
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
            "friction" => Some(self.friction.to_scientific_string()).as_deref(),
            "roughness" => self.roughness.map(|v| v.to_scientific_string()).as_deref(),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
            "surface" => Some(self.surface.as_str()),
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
