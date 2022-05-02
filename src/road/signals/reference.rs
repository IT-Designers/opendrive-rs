use std::borrow::Cow;

/// Provides a means to link a signal to a series of other elements (for example, objects and
/// signals).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Reference {
    /// Unique ID of the linked element
    pub element_id: String,
    /// Type of the linked element
    pub element_type: ElementType,
    /// Type of the linkage
    /// Free text, depending on application
    pub r#type: Option<String>,
}

impl Reference {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "elementId" => Some(self.element_id.as_str()),
            "elementType" => Some(self.element_type.as_str()),
            "type" => self.r#type.as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Reference
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            element_id: read.attribute("elementId")?,
            element_type: read.attribute("elementType")?,
            r#type: read.attribute_opt("type")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum ElementType {
    Object,
    Signal,
}

impl_from_str_as_str!(
    ElementType,
    "object" => Object,
    "signal" => Signal
);
