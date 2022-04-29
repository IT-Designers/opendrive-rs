use crate::road::railroad::platform::Platform;
use std::borrow::Cow;
use vec1::Vec1;

/// Defines stations for tram and railroad applications and for automotive environments. May refer
/// to multiple tracks and is therefore defined on the same level as junctions.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Station {
    pub platform: Vec1<Platform>,
    /// Unique ID within database
    pub id: String,
    /// Unique name of the station
    pub name: String,
    /// Type of station. Free text, depending on the application.
    /// e.g.: small, medium, large
    pub r#type: Option<StationType>,
}

impl Station {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "id" => Some(self.id.as_str()),
            "name" => Some(self.name.as_str()),
            "type" => self.r#type.as_ref().map(StationType::as_str),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for platform in &self.platform {
            visit_children!(visitor, "platform" => platform);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Station
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut platform = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "platform" true => Platform => |v| platform.push(v),
        );

        Ok(Self {
            platform: Vec1::try_from_vec(platform).unwrap(),
            id: read.attribute("id")?,
            name: read.attribute("name")?,
            r#type: read.attribute_opt("type")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum StationType {
    Small,
    Medium,
    Large,
}

impl_from_str_as_str!(
    StationType,
    "small" => Small,
    "medium" => Medium,
    "large" => Large,
);
