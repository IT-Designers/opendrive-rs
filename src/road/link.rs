use crate::core::additional_data::AdditionalData;
use crate::road::predecessor_successor::PredecessorSuccessor;
use std::borrow::Cow;

/// Follows the road header if the road is linked to a successor or a predecessor. Isolated roads
/// may omit this element.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Link {
    pub predecessor: Option<PredecessorSuccessor>,
    pub successor: Option<PredecessorSuccessor>,
    pub additional_data: AdditionalData,
}

impl Link {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(visitor)
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(predecessor) = &self.predecessor {
            visit_children!(visitor, "predecessor" => predecessor);
        }

        if let Some(successor) = &self.successor {
            visit_children!(visitor, "successor" => successor);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Link
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut predecessor = None;
        let mut successor = None;
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "predecessor" => PredecessorSuccessor => |v| predecessor = Some(v),
            "successor" => PredecessorSuccessor => |v| successor = Some(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            predecessor,
            successor,
            additional_data,
        })
    }
}
