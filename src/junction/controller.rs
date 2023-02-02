use std::borrow::Cow;

/// Lists the controllers that are used for the management of a junction.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Controller {
    /// ID of the controller
    pub id: String,
    /// Sequence number (priority) of this controller with respect to other controllers in the same
    /// junction
    pub sequence: Option<u64>,
    /// Type of control for this junction. Free text, depending on the application.
    pub r#type: Option<String>,
}

impl Controller {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "id" => Some(self.id.as_str()),
            "sequence" => self.sequence.map(|v| v.to_string()).as_deref(),
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
impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Controller
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            id: read.attribute("id")?,
            sequence: read.attribute_opt("sequence")?,
            r#type: read.attribute_opt("type")?,
        })
    }
}
