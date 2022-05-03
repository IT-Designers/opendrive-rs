use crate::core::additional_data::AdditionalData;
use crate::lane::lane_type::LaneType;
use crate::object::corner::Corner;
use crate::object::corner_local::CornerLocal;
use crate::object::corner_road::CornerRoad;
use crate::object::outline_fill_type::OutlineFillType;
use std::borrow::Cow;
use vec1::Vec1;

/// Defines a series of corner points, including the height of the object relative to the road
/// reference line. For areas, the points should be listed in counter-clockwise order.
/// An `<outline>` element shall be followed by one or more `<cornerRoad>` element or by one or more
/// `<cornerLocal>` element.
/// ASAM OpenDRIVE 1.4 outline definitions (without `<outlines>` parent element) shall still be
/// supported.
#[derive(Debug, Clone, PartialEq)]
pub struct Outline {
    /// If true, the outline describes an area, not a linear feature
    pub closed: Option<bool>,
    /// Type used to fill the area inside the outline
    pub fill_type: Option<OutlineFillType>,
    /// ID of the outline. Must be unique within one object.
    pub id: Option<u64>,
    /// Describes the lane type of the outline
    pub lane_type: Option<LaneType>,
    /// Defines if outline is an outer outline of the object
    pub outer: Option<bool>,
    pub choice: Vec1<Corner>,
    pub additional_data: AdditionalData,
}

impl Outline {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "closed" => self.closed.map(|v| v.to_string()).as_deref(),
            "fillType" => self.fill_type.as_ref().map(OutlineFillType::as_str),
            "id" => self.id.map(|v| v.to_string()).as_deref(),
            "laneType" => self.lane_type.as_ref().map(LaneType::as_str),
            "outer" => self.outer.map(|v| v.to_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for choice in &self.choice {
            match choice {
                Corner::Road(road) => visit_children!(visitor, "cornerRoad" => road),
                Corner::Local(local) => visit_children!(visitor, "cornerLocal" => local),
            }
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Outline
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut choice = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "cornerRoad" => CornerRoad => |v| choice.push(Corner::Road(v)),
            "cornerLocal" => CornerLocal => |v| choice.push(Corner::Local(v)),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            closed: read.attribute_opt("closed")?,
            fill_type: read.attribute_opt("fillType")?,
            id: read.attribute_opt("id")?,
            lane_type: read.attribute_opt("laneType")?,
            outer: read.attribute_opt("outer")?,
            choice: Vec1::try_from_vec(choice).map_err(|_| {
                crate::parser::Error::missing_element(
                    read.path().to_string(),
                    "cornerRoad|cornerLocal",
                    core::any::type_name::<Corner>(),
                )
            })?,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Outline {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            closed: u.arbitrary()?,
            fill_type: u.arbitrary()?,
            id: u.arbitrary()?,
            lane_type: u.arbitrary()?,
            outer: u.arbitrary()?,
            choice: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            additional_data: u.arbitrary()?,
        })
    }
}
