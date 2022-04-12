use std::borrow::Cow;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Describes the material properties of objects, for example, patches that are part of the road
/// surface but deviate from the standard road material. Supersedes the material specified in the
/// `<road material>` element and is valid only within the outline of the parent road object.
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    /// Friction value, depending on application
    pub friction: Option<f64>,
    /// Roughness, for example, for sound and motion systems, depending on application
    pub roughness: Option<f64>,
    /// Surface material code, depending on application
    pub surface: Option<String>,
}

impl Material {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            friction: find_map_parse_attr!(attributes, "friction", Option<f64>)?,
            roughness: find_map_parse_attr!(attributes, "roughness", Option<f64>)?,
            surface: find_map_parse_attr!(attributes, "surface", Option<String>)?,
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
            "friction" => self.friction.map(|v| v.to_scientific_string()).as_deref(),
            "roughness" => self.roughness.map(|v| v.to_scientific_string()).as_deref(),
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

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Material {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            friction: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?,
            roughness: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?,
            surface: u.arbitrary()?,
        })
    }
}
