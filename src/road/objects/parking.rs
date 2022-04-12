use std::borrow::Cow;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Details for a parking space may be added to the `<object>` element.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct ParkingSpace {
    /// Access definitions for the parking space. Parking spaces tagged with "women" and
    /// "handicapped" are vehicles of type car.
    pub access: Access,
    /// Free text, depending on application
    pub restrictions: Option<String>,
}

impl ParkingSpace {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            access: find_map_parse_attr!(attributes, "access", Access)?,
            restrictions: find_map_parse_attr!(attributes, "restrictions", Option<String>)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "access" => Some(self.access.as_str()),
            "restrictions" => self.restrictions.as_deref(),
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Access {
    All,
    Car,
    Women,
    Handicapped,
    Bus,
    Truck,
    Electric,
    Residents,
}

impl_from_str_as_str!(
    Access,
    "all" => All,
    "car" => Car,
    "women" => Women,
    "handicapped" => Handicapped,
    "bus" => Bus,
    "truck" => Truck,
    "electric" => Electric,
    "residents" => Residents,
);
