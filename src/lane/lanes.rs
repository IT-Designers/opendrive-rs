use crate::core::additional_data::AdditionalData;
use crate::lane::lane_section::LaneSection;
use crate::lane::offset::Offset;
use std::borrow::Cow;
use vec1::Vec1;

/// Contains a series of lane section elements that define the characteristics of the road cross
/// sections with respect to the lanes along the reference line.
#[derive(Debug, Clone, PartialEq)]
pub struct Lanes {
    pub lane_offset: Vec<Offset>,
    pub lane_section: Vec1<LaneSection>,
    pub additional_data: AdditionalData,
}

impl Lanes {
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
        for lane_offset in &self.lane_offset {
            visit_children!(visitor, "laneOffset" => lane_offset);
        }

        for lane_section in &self.lane_section {
            visit_children!(visitor, "laneSection" => lane_section);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Lanes
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut lane_offset = Vec::new();
        let mut lane_section = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "laneOffset" => Offset => |v| lane_offset.push(v),
            "laneSection" true => LaneSection => |v| lane_section.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            lane_offset,
            lane_section: Vec1::try_from_vec(lane_section).unwrap(),
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Lanes {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            lane_offset: u.arbitrary()?,
            lane_section: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            additional_data: u.arbitrary()?,
        })
    }
}
