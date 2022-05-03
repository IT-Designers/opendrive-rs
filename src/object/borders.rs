use crate::core::additional_data::AdditionalData;
use crate::object::border::Border;
use std::borrow::Cow;
use vec1::Vec1;

/// Objects may have a border, that is, a frame of a defined width. Different border types are available.
#[derive(Debug, Clone, PartialEq)]
pub struct Borders {
    pub border: Vec1<Border>,
    pub additional_data: AdditionalData,
}

impl Borders {
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
        for border in &self.border {
            visit_children!(visitor, "border" => border);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Borders
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut border = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "border" true => Border => |v| border.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            border: Vec1::try_from_vec(border).unwrap(),
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Borders {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            border: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            additional_data: u.arbitrary()?,
        })
    }
}
