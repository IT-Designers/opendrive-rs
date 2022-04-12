use std::borrow::Cow;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct LaneValidity {
    /// Minimum ID of the lanes for which the object is valid
    pub from_lane: i64,
    /// Maximum ID of the lanes for which the object is valid
    pub to_lane: i64,
}

impl LaneValidity {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            from_lane: find_map_parse_attr!(attributes, "fromLane", i64)?,
            to_lane: find_map_parse_attr!(attributes, "toLane", i64)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "fromLane" => &self.from_lane.to_string(),
            "toLane" => &self.to_lane.to_string(),
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
