use crate::junction::crg_mode::CrgMode;
use crate::junction::crg_purpose::CrgPurpose;
use crate::object::orientation::Orientation;
use std::borrow::Cow;
use uom::si::angle::radian;
use uom::si::f64::Angle;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Data described in OpenCRG is represented by the `<CRG>` element within the `<surface>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Crg {
    /// Name of the file containing the CRG data
    pub file: String,
    /// Heading offset between CRG center line and reference line of the road (only allowed for mode
    /// genuine, default = 0.0).
    pub h_offset: Option<Angle>,
    /// Attachment mode for the surface data, see specification.
    pub mode: CrgMode,
    /// Orientation of the CRG data set relative to the parent `<road>` element. Only allowed for
    /// mode attached and attached0.
    pub orientation: Orientation,
    /// Physical purpose of the data contained in the CRG file; if the attribute is missing, data
    /// will be interpreted as elevation data.
    pub purpose: Option<CrgPurpose>,
    /// End of the application of CRG (s-coordinate)
    pub s_end: Length,
    /// s-offset between CRG center line and reference line of the road (default = 0.0)
    pub s_offset: Option<Length>,
    /// Start of the application of CRG data (s-coordinate)
    pub s_start: Length,
    /// t-offset between CRG center line and reference line of the road (default = 0.0)
    pub t_offset: Option<Length>,
    /// z-offset between CRG center line and reference line of the road (default = 0.0). Only allowed for purpose elevation.
    pub z_offset: Option<Length>,
    /// z-scale factor for the surface description (default = 1.0). Only allowed for purpose
    /// elevation.
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
            "h_offset" => self.h_offset.map(|v| v.value.to_scientific_string()).as_deref(),
            "mode" => Some(self.mode.as_str()),
            "orientation" => Some(self.orientation.as_str()),
            "purpose" => self.purpose.as_ref().map(CrgPurpose::as_str),
            "sEnd" => Some(self.s_end.value.to_scientific_string()).as_deref(),
            "sOffset" => self.s_offset.map(|v| v.value.to_scientific_string()).as_deref(),
            "sStart" => Some(self.s_start.value.to_scientific_string()).as_deref(),
            "tOffset" => self.t_offset.map(|v| v.value.to_scientific_string()).as_deref(),
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
            h_offset: read.attribute_opt("hOffset")?.map(Angle::new::<radian>),
            mode: read.attribute("mode")?,
            orientation: read.attribute("orientation")?,
            purpose: read.attribute_opt("purpose")?,
            s_end: read.attribute("sEnd").map(Length::new::<meter>)?,
            s_offset: read.attribute_opt("sOffset")?.map(Length::new::<meter>),
            s_start: read.attribute("sStart").map(Length::new::<meter>)?,
            t_offset: read.attribute_opt("tOffset")?.map(Length::new::<meter>),
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
            h_offset: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Angle::new::<radian>),
            mode: u.arbitrary()?,
            orientation: u.arbitrary()?,
            purpose: u.arbitrary()?,
            s_end: u.not_nan_f64().map(Length::new::<meter>)?,
            s_offset: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            s_start: u.not_nan_f64().map(Length::new::<meter>)?,
            t_offset: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
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
