use crate::core::additional_data::AdditionalData;
use crate::lane::type_link::TypeLine;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use vec1::Vec1;

/// Each type definition shall contain one or more line definitions with additional information
/// about the lines that the road mark is composed of.
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub line: Vec1<TypeLine>,
    /// Name of the road mark type. May be chosen freely.
    pub name: String,
    /// Accumulated width of the road mark. In case of several `<line>` elements this @width is the
    /// sum of all @width of `<line>` elements and spaces in between, necessary to form the road
    /// mark. This attribute supersedes the definition in the `<roadMark>` element.
    pub width: Length,
    pub additional_data: AdditionalData,
}

impl Type {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "name" => &self.name,
            "width" => &self.width.value.to_scientific_string(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for line in &self.line {
            visit_children!(visitor, "line" => line);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Type
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut line = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "line" true => TypeLine => |v| line.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            line: Vec1::try_from_vec(line).unwrap(),
            name: read.attribute("name")?,
            width: read.attribute("width").map(Length::new::<meter>)?,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Type {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            line: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            name: u.arbitrary()?,
            width: Length::new::<meter>(u.not_nan_f64()?),
            additional_data: u.arbitrary()?,
        })
    }
}
