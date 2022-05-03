use crate::core::additional_data::AdditionalData;
use crate::lane::center::Center;
use crate::lane::left::Left;
use crate::lane::right::Right;
use std::borrow::Cow;

/// Lanes may be split into multiple lane sections. Each lane section contains a fixed number of
/// lanes. Every time the number of lanes changes, a new lane section is required. The distance
/// between two succeeding lane sections shall not be zero.
#[derive(Debug, Clone, PartialEq)]
pub struct LaneSection {
    /// s-coordinate of start position
    pub s: f64,
    /// Lane section element is valid for one side only (left, center, or right), depending on the
    /// child elements.
    pub single_side: Option<bool>,
    pub left: Option<Left>,
    pub center: Center,
    pub right: Option<Right>,
    pub additional_data: AdditionalData,
}

impl LaneSection {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "s" => Some(self.s.to_scientific_string()).as_deref(),
            "singleSide" => self.single_side.map(|b| b.to_string()).as_deref()
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(left) = &self.left {
            visit_children!(visitor, "left" => left);
        }

        visit_children!(visitor, "center" => self.center);

        if let Some(right) = &self.right {
            visit_children!(visitor, "right" => right);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for LaneSection
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut left = None;
        let mut center = None;
        let mut right = None;
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "left" => Left => |v| left = Some(v),
            "center" true => Center => |v| center = Some(v),
            "right" => Right => |v| right = Some(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            s: read.attribute("s")?,
            single_side: read.attribute_opt("singleSide")?,
            left,
            center: center.unwrap(),
            right,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for LaneSection {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            s: u.not_nan_f64()?,
            single_side: u.arbitrary()?,
            left: u.arbitrary()?,
            center: u.arbitrary()?,
            right: u.arbitrary()?,
            additional_data: u.arbitrary()?,
        })
    }
}
