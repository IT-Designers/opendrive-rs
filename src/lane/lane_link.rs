use crate::core::additional_data::AdditionalData;
use crate::lane::predecessor_successor::PredecessorSuccessor;
use std::borrow::Cow;

/// For links between lanes with an identical reference line, the lane predecessor and successor
/// information provide the IDs of lanes on the preceding or following lane section.
/// For links between lanes with different reference line,  the lane predecessor and successor
/// information provide the IDs of lanes on the first or last lane section of the other reference
/// line depending on the contact point of the road linkage.
/// This element may only be omitted, if lanes end at a junction or have no physical link.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct LaneLink {
    pub predecessor: Vec<PredecessorSuccessor>,
    pub successor: Vec<PredecessorSuccessor>,
    pub additional_data: AdditionalData,
}

impl LaneLink {
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
        for predecessor in &self.predecessor {
            visit_children!(visitor, "predecessor" => predecessor);
        }

        for successor in &self.successor {
            visit_children!(visitor, "successor" => successor);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for LaneLink
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut predecessor = Vec::new();
        let mut successor = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "predecessor" => PredecessorSuccessor => |v| predecessor.push(v),
            "successor" => PredecessorSuccessor => |v| successor.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            predecessor,
            successor,
            additional_data,
        })
    }
}
