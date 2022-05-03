use crate::junction::crg_mode::CrgMode;
use crate::junction::crg_purpose::CrgPurpose;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Data described in OpenCRG are represented by the `<CRG>` element within the `<surface>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Crg {
    /// Name of the file containing the CRG data
    pub file: String,
    /// Attachment mode for the surface data.
    pub mode: CrgMode,
    /// Physical purpose of the data contained in the CRG file; if the attribute is missing, data
    /// will be interpreted as elevation data.
    pub purpose: Option<CrgPurpose>,
    /// z offset between CRG center line and inertial xy-plane (default = 0.0
    pub z_offset: Option<Length>,
    /// z scale factor for the surface description (default = 1.0)
    pub z_scale: Option<f64>,
}

impl Crg {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "file" => Some(self.file.as_str()),
            "mode" => Some(self.mode.as_str()),
            "purpose" => self.purpose.as_ref().map(CrgPurpose::as_str),
            "zOffset" => self.z_offset.map(|v| v.value.to_scientific_string()).as_deref(),
            "zScale" => self.z_scale.map(|v| v.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Crg
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            file: read.attribute("file")?,
            mode: read.attribute("mode")?,
            purpose: read.attribute_opt("purpose")?,
            z_offset: read.attribute_opt("zOffset")?.map(Length::new::<meter>),
            z_scale: read.attribute_opt("zScale")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Crg {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            file: u.arbitrary()?,
            mode: u.arbitrary()?,
            purpose: u.arbitrary()?,
            z_offset: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            z_scale: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?,
        })
    }
}
