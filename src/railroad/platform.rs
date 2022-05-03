use crate::core::additional_data::AdditionalData;
use crate::railroad::segment::Segment;
use std::borrow::Cow;
use vec1::Vec1;

/// Each `<station>` element must contain at least one `<platform>` element. Each `<platform>`
/// element must contain at least one reference to a valid track segment.
#[derive(Debug, Clone, PartialEq)]
pub struct Platform {
    pub segment: Vec1<Segment>,
    /// Unique ID within database
    pub id: String,
    /// Name of the platform. May be chosen freely.
    pub name: Option<String>,
    pub additional_data: AdditionalData,
}

impl Platform {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "id" => Some(self.id.as_str()),
            "name" => self.name.as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for segment in &self.segment {
            visit_children!(visitor, "segment" => segment);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Platform
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut segment = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "segment" true => Segment => |v| segment.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            segment: Vec1::try_from_vec(segment).unwrap(),
            id: read.attribute("id")?,
            name: read.attribute_opt("name")?,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Platform {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            segment: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            id: u.arbitrary()?,
            name: u.arbitrary()?,
            additional_data: u.arbitrary()?,
        })
    }
}
