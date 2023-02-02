use std::borrow::Cow;

/// Provides information about the lanes that are linked between an incoming road and a connecting
/// road. It is strongly recommended to provide this element. It is deprecated to omit the
/// `<laneLink>` element.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct LaneLink {
    /// ID of the incoming lane
    pub from: i64,
    /// ID of the connection lane
    pub to: i64,
}

impl LaneLink {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "from" => &self.from.to_string(),
            "to" => &self.to.to_string(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for LaneLink
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            from: read.attribute("from")?,
            to: read.attribute("to")?,
        })
    }
}
