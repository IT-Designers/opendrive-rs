use crate::core::additional_data::AdditionalData;
use crate::railroad::platform::Platform;
use crate::railroad::station_type::StationType;
use std::borrow::Cow;
use vec1::Vec1;

/// Defines stations for tram and railroad applications and for automotive environments. May refer
/// to multiple tracks and is therefore defined on the same level as junctions.
#[derive(Debug, Clone, PartialEq)]
pub struct Station {
    pub platform: Vec1<Platform>,
    /// Unique ID within database
    pub id: String,
    /// Unique name of the station
    pub name: String,
    /// Type of station. Free text, depending on the application.
    /// e.g.: small, medium, large
    pub r#type: Option<StationType>,
    pub additional_data: AdditionalData,
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

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Station
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut platform = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "platform" true => Platform => |v| platform.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            platform: Vec1::try_from_vec(platform).unwrap(),
            id: read.attribute("id")?,
            name: read.attribute("name")?,
            r#type: read.attribute_opt("type")?,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Station {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            platform: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            id: u.arbitrary()?,
            name: u.arbitrary()?,
            r#type: u.arbitrary()?,
            additional_data: u.arbitrary()?,
        })
    }
}
