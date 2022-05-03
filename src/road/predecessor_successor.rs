use crate::junction::contact_point::ContactPoint;
use crate::junction::element_dir::ElementDir;
use crate::road::element_type::ElementType;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Successors and predecessors can be junctions or roads. For each, different attribute sets shall
/// be used.
#[derive(Debug, Clone, PartialEq)]
pub struct PredecessorSuccessor {
    /// Contact point of link on the linked element
    pub contact_point: Option<ContactPoint>,
    /// To be provided when elementS is used for the connection definition. Indicates the direction
    /// on the predecessor from which the road is entered.
    pub element_dir: Option<ElementDir>,
    /// ID of the linked element
    pub element_id: String,
    /// Alternative to contactPoint for virtual junctions. Indicates a connection within the
    /// predecessor, meaning not at the start or end of the predecessor. Shall only be used for
    /// elementType "road"
    pub element_s: Option<Length>,
    /// Type of the linked element
    pub element_type: Option<ElementType>,
}

impl PredecessorSuccessor {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "contactPoint" => self.contact_point.as_ref().map(ContactPoint::as_str),
            "elementDir" => self.element_dir.as_ref().map(ElementDir::as_str),
            "elementId" => Some(self.element_id.as_str()),
            "elementS" => self.element_s.map(|v| v.value.to_scientific_string()).as_deref(),
            "elementType" => self.element_type.as_ref().map(ElementType::as_str),
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
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            contact_point: read.attribute_opt("contactPoint")?,
            element_dir: read.attribute_opt("elementDir")?,
            element_id: read.attribute("elementId")?,
            element_s: read.attribute_opt("elementS")?.map(Length::new::<meter>),
            element_type: read.attribute_opt("elementType")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for PredecessorSuccessor {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            contact_point: u.arbitrary()?,
            element_dir: u.arbitrary()?,
            element_id: u.arbitrary()?,
            element_s: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
            element_type: u.arbitrary()?,
        })
    }
}
