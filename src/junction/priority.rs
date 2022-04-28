use std::borrow::Cow;

/// The junction priority record provides information about the priority of a connecting road over
/// another connecting road. It is only required if priorities cannot be derived from signs or
/// signals in a junction or on tracks leading to a junction.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Priority {
    /// ID of the prioritized connecting road
    pub high: Option<String>,
    /// ID of the connecting road with lower priority
    pub low: Option<String>,
}

impl Priority {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "high" => self.high.as_deref(),
            "low" => self.low.as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Priority
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            high: read.attribute_opt("high")?,
            low: read.attribute_opt("low")?,
        })
    }
}
