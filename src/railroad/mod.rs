use crate::core::additional_data::AdditionalData;
use std::borrow::Cow;
use switch::Switch;

pub mod main_track;
pub mod partner;
pub mod platform;
pub mod segment;
pub mod segment_side;
pub mod side_track;
pub mod station;
pub mod station_type;
pub mod switch;
pub mod switch_position;

/// Container for all railroad definitions that shall be applied along a road.
/// The available set of railroad elements is currently limited to the definition of switches. All
/// other entries shall be covered with the existing elements, for example, track definition by
/// `<road>`, signal definition by `<signal>`, etc. Railroad-specific elements are defined against
/// the background of streetcar applications.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Railroad {
    pub switch: Vec<Switch>,
    pub additional_data: AdditionalData,
}

impl Railroad {
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
        for switch in &self.switch {
            visit_children!(visitor, "switch" => switch);
        }

        self.additional_data.append_children(visitor)
    }
}
impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Railroad
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut switch = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "switch" => Switch => |v| switch.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            switch,
            additional_data,
        })
    }
}
