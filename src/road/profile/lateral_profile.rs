use crate::core::additional_data::AdditionalData;
use crate::road::profile::shape::Shape;
use crate::road::profile::super_elevation::SuperElevation;
use std::borrow::Cow;

/// Contains a series of superelevation elements that define the characteristics of the road
/// surface's banking along the reference line.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct LateralProfile {
    pub super_elevation: Vec<SuperElevation>,
    pub shape: Vec<Shape>,
    pub additional_data: AdditionalData,
}

impl LateralProfile {
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
        for elevation in &self.super_elevation {
            visit_children!(visitor, "superelevation" => elevation);
        }

        for shape in &self.shape {
            visit_children!(visitor, "shape" => shape);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for LateralProfile
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut super_elevation = Vec::new();
        let mut shape = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "superelevation" => SuperElevation => |v| super_elevation.push(v),
            "shape" => Shape => |v| shape.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            super_elevation,
            shape,
            additional_data,
        })
    }
}
