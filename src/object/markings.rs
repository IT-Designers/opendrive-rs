use crate::core::additional_data::AdditionalData;
use crate::object::marking::Marking;
use std::borrow::Cow;
use vec1::Vec1;

/// Describes the appearance of the parking space with multiple marking elements.
#[derive(Debug, Clone, PartialEq)]
pub struct Markings {
    pub marking: Vec1<Marking>,
    pub additional_data: AdditionalData,
}

impl Markings {
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
        for marking in &self.marking {
            visit_children!(visitor, "marking" => marking);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Markings
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut marking = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "marking" true => Marking => |v| marking.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            marking: Vec1::try_from_vec(marking).unwrap(),
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Markings {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            marking: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            additional_data: u.arbitrary()?,
        })
    }
}
