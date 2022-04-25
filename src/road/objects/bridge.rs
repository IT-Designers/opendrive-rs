use crate::road::objects::validity::LaneValidity;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Bridge
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut validity = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "validity" => LaneValidity => |v| validity.push(v),
        );

        Ok(Self {
            id: read.attribute("id")?,
            length: read.attribute("length").map(Length::new::<meter>)?,
            name: read.attribute_opt("name")?,
            s: read.attribute("s").map(Length::new::<meter>)?,
            r#type: read.attribute("type")?,
            validity,
        })
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
