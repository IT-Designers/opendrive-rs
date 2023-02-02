use std::borrow::Cow;

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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Material
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            friction: read.attribute_opt("friction")?,
            roughness: read.attribute_opt("roughness")?,
            surface: read.attribute_opt("surface")?,
        })
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
