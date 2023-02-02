use crate::core::additional_data::AdditionalData;
use crate::object::outline::Outline;
use std::borrow::Cow;
use vec1::Vec1;

/// An outline defines a series of corner points, including the height of the object relative to the
/// road reference line. The inner area of the described outline may be filled with a filling type,
/// such as grass, concrete, asphalt, or pavement.
#[derive(Debug, Clone, PartialEq)]
pub struct Outlines {
    pub outline: Vec1<Outline>,
    pub additional_data: AdditionalData,
}

impl Outlines {
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
        for outline in &self.outline {
            visit_children!(visitor, "outline" => outline);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Outlines
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut outline = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "outline" true => Outline => |v| outline.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            outline: Vec1::try_from_vec(outline).unwrap(),
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Outlines {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            outline: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            additional_data: u.arbitrary()?,
        })
    }
}
