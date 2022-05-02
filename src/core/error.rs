use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// The absolute or relative errors of road data are described by `<error>` elements within the
/// `<dataQuality>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    /// Absolute error of the road data in x/y direct
    pub xy_absolute: Length,
    /// Relative error of the road data in x/y direction
    pub xy_relative: Length,
    /// Absolute error of the road data in z direction
    pub z_absolute: Length,
    /// Relative error of the road data in z direction
    pub z_relative: Length,
}

impl Error {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "xyAbsolute" => &self.xy_absolute.value.to_scientific_string(),
            "xyRelative" => &self.xy_relative.value.to_scientific_string(),
            "zAbsolute" => &self.z_absolute.value.to_scientific_string(),
            "zRelative" => &self.z_relative.value.to_scientific_string(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Error
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            xy_absolute: Length::new::<meter>(read.attribute("xyAbsolute")?),
            xy_relative: Length::new::<meter>(read.attribute("xyRelative")?),
            z_absolute: Length::new::<meter>(read.attribute("zAbsolute")?),
            z_relative: Length::new::<meter>(read.attribute("zRelative")?),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Error {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            xy_absolute: Length::new::<meter>(u.not_nan_f64()?),
            xy_relative: Length::new::<meter>(u.not_nan_f64()?),
            z_absolute: Length::new::<meter>(u.not_nan_f64()?),
            z_relative: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}
