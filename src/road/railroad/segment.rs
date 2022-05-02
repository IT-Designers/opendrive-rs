use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Each `<platform>` element is valid on one or more track segments. The `<segment>` element must
/// be specified.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    /// Unique ID of the `<road>` element (track) that accompanies the platform
    pub road_id: String,
    /// Maximum s-coordinate on `<road>` element that has an adjacent platform
    pub s_end: Length,
    /// Side of track on which the platform is situated when going from sStart to sEnd
    pub side: SegmentSide,
    /// Minimum s-coordinate on `<road>` element that has an adjacent platform
    pub s_start: Length,
}

impl Segment {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "roadId" => self.road_id.as_str(),
            "sEnd" => self.s_end.value.to_scientific_string().as_str(),
            "side" => self.side.as_str(),
            "sStart" => self.s_start.value.to_scientific_string().as_str(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Segment
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            road_id: read.attribute("roadId")?,
            s_end: read.attribute("sEnd").map(Length::new::<meter>)?,
            side: read.attribute("side")?,
            s_start: read.attribute("sStart").map(Length::new::<meter>)?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Segment {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            road_id: u.arbitrary()?,
            s_end: Length::new::<meter>(u.not_nan_f64()?),
            side: u.arbitrary()?,
            s_start: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum SegmentSide {
    Left,
    Right,
}

impl_from_str_as_str!(
    SegmentSide,
    "left" => Left,
    "right" => Right,
);
