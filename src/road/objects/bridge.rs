use crate::road::objects::validity::LaneValidity;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

#[derive(Debug, Clone, PartialEq)]
pub struct Bridge {
    /// Unique ID within database
    pub id: String,
    /// Length of the bridge (in s-direction)
    pub length: Length,
    /// Name of the bridge. May be chosen freely.
    pub name: Option<String>,
    /// s-coordinate
    pub s: Length,
    /// Type of bridge
    pub r#type: BridgeType,
    pub validity: Vec<LaneValidity>,
}

impl Bridge {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut validity = Vec::new();

        find_map_parse_elem!(
            events,
            "validity" => |attributes| {
                validity.push(LaneValidity::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self {
            id: find_map_parse_attr!(attributes, "id", String)?,
            length: find_map_parse_attr!(attributes, "length", f64).map(Length::new::<meter>)?,
            name: find_map_parse_attr!(attributes, "name", Option<String>)?,
            s: find_map_parse_attr!(attributes, "s", f64).map(Length::new::<meter>)?,
            r#type: find_map_parse_attr!(attributes, "type", BridgeType)?,
            validity,
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
            "id" => Some(self.id.as_str()),
            "length" => Some(self.length.value.to_scientific_string()).as_deref(),
            "name" => self.name.as_deref(),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "type" => Some(self.r#type.as_str()),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for validity in &self.validity {
            visit_children!(visitor, "validity" => validity);
        }
        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Bridge {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            id: u.arbitrary()?,
            length: Length::new::<meter>(u.not_nan_f64()?),
            name: u.arbitrary()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
            r#type: u.arbitrary()?,
            validity: u.arbitrary()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum BridgeType {
    Concrete,
    Steel,
    Brick,
    Wood,
}

impl_from_str_as_str!(
    BridgeType,
    "concrete" => Concrete,
    "steel" => Steel,
    "brick" => Brick,
    "wood" => Wood,
);
