use crate::core::additional_data::AdditionalData;
use crate::road::crg::Crg;
use std::borrow::Cow;

/// In ASAM OpenDRIVE, the road surface is represented by the `<surface>` element within the
/// `<road>` element.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Surface {
    pub crg: Vec<Crg>,
    pub additional_data: AdditionalData,
}

impl Surface {
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
        for crg in &self.crg {
            visit_children!(visitor, "CRG" => crg);
        }

        self.additional_data.append_children(visitor)
    }
}
impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Surface
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut crg = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "CRG" => Crg => |v| crg.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            crg,
            additional_data,
        })
    }
}
