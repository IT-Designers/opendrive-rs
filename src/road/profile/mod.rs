use crate::core::additional_data::AdditionalData;
use elevation::Elevation;
use std::borrow::Cow;

pub mod elevation;
pub mod lateral_profile;
pub mod shape;
pub mod super_elevation;

/// Defines the characteristics of the road elevation along the reference line.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct ElevationProfile {
    pub elevation: Vec<Elevation>,
    pub additional_data: AdditionalData,
}

impl ElevationProfile {
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
        for elevation in &self.elevation {
            visit_children!(visitor, "elevation" => elevation);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for ElevationProfile
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut elevation = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "elevation" => Elevation => |v| elevation.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            elevation,
            additional_data,
        })
    }
}
