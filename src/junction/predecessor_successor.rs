use crate::junction::element_dir::ElementDir;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Provides detailed information about the predecessor / successor road of a virtual connection.
/// Currently, only the @elementType “road” is allowed.
#[derive(Debug, Clone, PartialEq)]
pub struct PredecessorSuccessor {
    /// Direction, relative to the s-direction, of the connection on the preceding / succeeding road
    pub element_dir: ElementDir,
    /// ID of the linked element
    pub element_id: String,
    /// s-coordinate where the connection meets the preceding / succeding road.
    pub element_s: Length,
    /// Type of the linked element. Currently only "road" is allowed.
    pub element_type: String,
}

impl PredecessorSuccessor {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "elementDir" => self.element_dir.as_str(),
            "elementId" => self.element_id.as_str(),
            "elementS" => self.element_s.value.to_scientific_string().as_str(),
            "elementType" => self.element_type.as_str(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for PredecessorSuccessor
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            element_dir: read.attribute("elementDir")?,
            element_id: read.attribute("elementId")?,
            element_s: read.attribute("elementS").map(Length::new::<meter>)?,
            element_type: read.attribute("elementType")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for PredecessorSuccessor {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            element_dir: u.arbitrary()?,
            element_id: u.arbitrary()?,
            element_s: Length::new::<meter>(u.not_nan_f64()?),
            element_type: u.arbitrary()?,
        })
    }
}
