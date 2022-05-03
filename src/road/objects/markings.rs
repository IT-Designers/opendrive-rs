use crate::lane::road_mark::weight::Weight;
use crate::road::objects::borders::CornerReference;
use crate::road::objects::road_mark_color::RoadMarkColor;
use crate::road::objects::side_type::SideType;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use vec1::Vec1;

/// Describes the appearance of the parking space with multiple marking elements.
#[derive(Debug, Clone, PartialEq)]
pub struct Markings {
    pub marking: Vec1<Marking>,
}

impl Markings {
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
        for marking in &self.marking {
            visit_children!(visitor, "marking" => marking);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Markings
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut marking = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "marking" true => Marking => |v| marking.push(v),
        );

        Ok(Self {
            marking: Vec1::try_from_vec(marking).unwrap(),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Markings {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            marking: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
        })
    }
}

/// Specifies a marking that is either attached to one side of the object bounding box or
/// referencing outline points.
#[derive(Debug, Clone, PartialEq)]
pub struct Marking {
    pub corner_reference: Vec<CornerReference>,
    /// Color of the marking
    pub color: RoadMarkColor,
    /// Length of the visible part
    pub line_length: Length,
    /// Side of the bounding box described in `<object>` element in the local coordinate system u/v
    pub side: Option<SideType>,
    /// Length of the gap between the visible parts
    pub space_length: Length,
    /// Lateral offset in u-direction from start of bounding box side where the first marking starts
    pub start_offset: Length,
    /// Lateral offset in u-direction from end of bounding box side where the marking ends
    pub stop_offset: Length,
    /// Optical "weight" of the marking
    pub weight: Option<Weight>,
    /// Width of the marking
    pub width: Option<Length>,
    /// Height of road mark above the road, i.e. thickness of the road mark
    pub z_offset: Option<Length>,
}

impl Marking {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "color" => Some(self.color.as_str()),
            "lineLength" => Some(self.line_length.value.to_scientific_string()).as_deref(),
            "side" => self.side.as_ref().map(SideType::as_str),
            "spaceLength" => Some(self.space_length.value.to_scientific_string()).as_deref(),
            "startOffset" => Some(self.start_offset.value.to_scientific_string()).as_deref(),
            "stopOffset" => Some(self.stop_offset.value.to_scientific_string()).as_deref(),
            "weight" => self.weight.as_ref().map(Weight::as_str),
            "width" => self.width.map(|v| v.value.to_scientific_string()).as_deref(),
            "zOffset" => self.z_offset.map(|v| v.value.to_scientific_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for corner_reference in &self.corner_reference {
            visit_children!(visitor, "cornerReference" => corner_reference);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Marking
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut corner_reference = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "cornerReference" => CornerReference => |v| corner_reference.push(v),
        );

        Ok(Self {
            corner_reference,
            color: read.attribute("color")?,
            line_length: read.attribute("lineLength").map(Length::new::<meter>)?,
            side: read.attribute_opt("side")?,
            space_length: read.attribute("spaceLength").map(Length::new::<meter>)?,
            start_offset: read.attribute("startOffset").map(Length::new::<meter>)?,
            stop_offset: read.attribute("stopOffset").map(Length::new::<meter>)?,
            weight: read.attribute_opt("weight")?,
            width: read.attribute_opt("width")?.map(Length::new::<meter>),
            z_offset: read.attribute_opt("zOffset")?.map(Length::new::<meter>),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Marking {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            corner_reference: u.arbitrary()?,
            color: u.arbitrary()?,
            line_length: Length::new::<meter>(u.not_nan_f64()?),
            side: u.arbitrary()?,
            space_length: Length::new::<meter>(u.not_nan_f64()?),
            start_offset: Length::new::<meter>(u.not_nan_f64()?),
            stop_offset: Length::new::<meter>(u.not_nan_f64()?),
            weight: u.arbitrary()?,
            width: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            z_offset: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
        })
    }
}
