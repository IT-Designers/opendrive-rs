use crate::junction::element_dir::ElementDir;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

#[derive(Debug, Clone, PartialEq)]
pub struct SideTrack {
    /// direction, relative to the s-direction, on the main track for entering the side track via
    /// the switch
    pub dir: ElementDir,
    /// Unique ID of the main track, that is, the `<road>` element. Must be consistent with parent
    /// containing this `<railroad>` element.
    pub id: String,
    /// s-coordinate of the switch, that is, the point where main track and side track meet
    pub s: Length,
}

impl SideTrack {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "dir" => self.dir.as_str(),
            "id" => self.id.as_str(),
            "s" => self.s.value.to_scientific_string().as_str(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for SideTrack
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            dir: read.attribute("dir")?,
            id: read.attribute("id")?,
            s: Length::new::<meter>(read.attribute("s")?),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for SideTrack {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            dir: u.arbitrary()?,
            id: u.arbitrary()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}
