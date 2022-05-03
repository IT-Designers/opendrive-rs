use crate::core::additional_data::AdditionalData;
use crate::junction::junction_group_type::JunctionGroupType;
use crate::junction::junction_reference::JunctionReference;
use std::borrow::Cow;
use vec1::Vec1;

/// Two or more junctions may be grouped in junction groups to indicate that these junctions belong
/// to the same roundabout.
/// The `<junctionGroup>` element is split into a header element and a series of member elements.
#[derive(Debug, Clone, PartialEq)]
pub struct JunctionGroup {
    pub junction_reference: Vec1<JunctionReference>,
    /// Unique ID within database
    pub id: String,
    /// Name of the junction group. May be chosen freely.
    pub name: Option<String>,
    /// Type of junction group
    pub r#type: JunctionGroupType,
    pub additional_data: AdditionalData,
}

impl JunctionGroup {
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
            "type" => Some(self.r#type.as_str()),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for junction_reference in &self.junction_reference {
            visit_children!(visitor, "junctionReference" => junction_reference);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for JunctionGroup
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut junction_reference = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "junctionReference" true => JunctionReference => |v| junction_reference.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            junction_reference: Vec1::try_from_vec(junction_reference).unwrap(),
            id: read.attribute("id")?,
            name: read.attribute_opt("name")?,
            r#type: read.attribute("type")?,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for JunctionGroup {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            junction_reference: {
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
