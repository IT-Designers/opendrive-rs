use crate::lane::Lane;
use std::borrow::Cow;

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct RightLane {
    /// ID of the lane
    pub id: i64,
    pub base: Lane,
}

impl RightLane {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        self.base.visit_attributes(|attributes| {
            let mut attributes = attributes.to_vec();
            let value = self.id.to_string();
            attributes.push(xml::attribute::Attribute::new(
                xml::name::Name::local("id"),
                &value,
            ));
            visitor(Cow::Owned(attributes))
        })
    }

    pub fn visit_children(
        &self,
        visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        self.base.visit_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for RightLane
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: read.attribute("id")?,
            base: Lane::try_from(read)?,
        })
    }
}
