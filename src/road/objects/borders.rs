use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use vec1::Vec1;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Objects may have a border, that is, a frame of a defined width. Different border types are available.
#[derive(Debug, Clone, PartialEq)]
pub struct Borders {
    pub border: Vec1<Border>,
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
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Borders
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut border = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "border" true => Border => |v| border.push(v),
        );

        Ok(Self {
            border: Vec1::try_from_vec(border).unwrap(),
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
        })
    }
}
/// Specifies a border along certain outline points.
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub corner_reference: Vec<CornerReference>,
    /// ID of the outline to use
    pub outline_id: u64,
    /// Appearance of border
    pub r#type: BorderType,
    /// Use all outline points for border. “true” is used as default.
    pub use_complete_outline: Option<bool>,
    /// Border width
    pub width: Length,
}

impl Border {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "outlineId" => Some(self.outline_id.to_string()).as_deref(),
            "type" => Some(self.r#type.as_str()),
            "useCompleteOutline" => self.use_complete_outline.map(|v| v.to_string()).as_deref(),
            "width" => Some(self.width.value.to_scientific_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for corner_reference in &self.corner_reference {
            visit_children!(visitor, "cornerReference" => corner_reference);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Border
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut corner_reference = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "cornerReference" => CornerReference => |v| corner_reference.push(v),
        );

        Ok(Self {
            outline_id: read.attribute("outlineId")?,
            r#type: read.attribute("type")?,
            use_complete_outline: read.attribute_opt("useCompleteOutline")?,
            width: read.attribute("width").map(Length::new::<meter>)?,
            corner_reference,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Border {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            corner_reference: u.arbitrary()?,
            outline_id: u.arbitrary()?,
            r#type: u.arbitrary()?,
            use_complete_outline: u.arbitrary()?,
            width: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}

/// Specifies a point by referencing an existing outline point.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct CornerReference {
    /// Identifier of the referenced outline point
    pub id: u64,
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for CornerReference
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: read.attribute("id")?,
        })
    }
}

impl CornerReference {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            id: find_map_parse_attr!(attributes, "id", u64)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "id" => &self.id.to_string(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(visitor);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum BorderType {
    Concrete,
    Curb,
}

impl_from_str_as_str!(
    BorderType,
    "concrete" => Concrete,
    "curb" => Curb,
);
