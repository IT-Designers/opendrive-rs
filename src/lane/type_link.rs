use crate::lane::road_mark::color::Color;
use crate::lane::road_mark::rule::Rule;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// A road mark may consist of one or more elements. Multiple elements are usually positioned
/// side-by-side. A line definition is valid for a given length of the lane and will be repeated
/// automatically.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeLine {
    /// Line color. If given, this attribute supersedes the definition in the `<roadMark>` element.
    pub color: Option<Color>,
    /// Length of the visible part
    pub length: Length,
    /// Rule that must be observed when passing the line from inside, for example, from the lane
    /// with the lower absolute ID to the lane with the higher absolute ID
    pub rule: Option<Rule>,
    /// Initial longitudinal offset of the line definition from the start of the road mark
    /// definition
    pub s_offset: Length,
    /// Length of the gap between the visible parts
    pub space: Length,
    /// Lateral offset from the lane border.
    /// If `<sway>` element is present, the lateral offset follows the sway.
    pub t_offset: Length,
    /// Line width
    pub width: Option<Length>,
}

impl TypeLine {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "color" => self.color.as_ref().map(Color::as_str),
            "length" => Some(self.length.value.to_scientific_string()).as_deref(),
            "rule" => self.rule.as_ref().map(Rule::as_str),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
            "space" => Some(self.space.value.to_scientific_string()).as_deref(),
            "tOffset" => Some(self.t_offset.value.to_scientific_string()).as_deref(),
            "width" => self.width.map(|v| v.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for TypeLine
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            color: read.attribute_opt("color")?,
            length: read.attribute("length").map(Length::new::<meter>)?,
            rule: read.attribute_opt("rule")?,
            s_offset: read.attribute("sOffset").map(Length::new::<meter>)?,
            space: read.attribute("space").map(Length::new::<meter>)?,
            t_offset: read.attribute("tOffset").map(Length::new::<meter>)?,
            width: read.attribute_opt("width")?.map(Length::new::<meter>),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for TypeLine {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            color: u.arbitrary()?,
            length: Length::new::<meter>(u.not_nan_f64()?),
            rule: u.arbitrary()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            space: Length::new::<meter>(u.not_nan_f64()?),
            t_offset: Length::new::<meter>(u.not_nan_f64()?),
            width: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
        })
    }
}
