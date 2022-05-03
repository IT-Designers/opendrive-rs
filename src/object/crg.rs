use std::borrow::Cow;

/// Elevation data described in {GLO_VAR_STA_ASAM_OpenCRG} are represented by the `<CRG>` element
/// within the `<surface>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Crg {
    /// Name of the file containing the CRG data.
    pub file: Option<String>,
    /// Determines if the object CRG hides the road surface CRG. Default is true.
    pub hide_road_surface_crg: Option<bool>,
    /// z-scale factor for the surface description (default = 1.0).
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
            "file" => self.file.as_deref(),
            "hideRoadSurfaceCRG" => self.hide_road_surface_crg.map(|v| v.to_string()).as_deref(),
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
            file: read.attribute_opt("file")?,
            hide_road_surface_crg: read.attribute_opt("hideRoadSurfaceCRG")?,
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
            hide_road_surface_crg: u.arbitrary()?,
            z_scale: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?,
        })
    }
}
