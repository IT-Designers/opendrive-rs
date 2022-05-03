use crate::core::additional_data::AdditionalData;
use crate::lane::road_mark::explicit_line::ExplicitLine;
use std::borrow::Cow;
use vec1::Vec1;

/// Irregular road markings that cannot be described by repetitive line patterns may be described by
/// individual road marking elements. These explicit definitions also contain `<line>` elements for
/// the line definition, however, these lines will not be repeated automatically as in repetitive
/// road marking types. In ASAM OpenDRIVE, irregular road marking types and lines are represented by
/// `<explicit>` elements within elements. The line definitions are contained in `<line>` elements
/// within the `<explicit>` element.
// The `<explicit>` element should specifically be used for measurement data.
#[derive(Debug, Clone, PartialEq)]
pub struct Explicit {
    pub line: Vec1<ExplicitLine>,
    pub additional_data: AdditionalData,
}

impl Explicit {
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
        for line in &self.line {
            visit_children!(visitor, "line" => line);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Explicit
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut line = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "line" true => ExplicitLine => |v| line.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            line: Vec1::try_from_vec(line).unwrap(),
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Explicit {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            line: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            additional_data: u.arbitrary()?,
        })
    }
}
