use crate::lane::road_mark::rule::Rule;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

#[derive(Debug, Clone, PartialEq)]
pub struct ExplicitLine {
    /// Length of the visible line
    pub length: Length,
    /// Rule that must be observed when passing the line from inside, that is, from the lane with
    /// the lower absolute ID to the lane with the higher absolute ID
    pub rule: Option<Rule>,
    /// Offset of start position of the `<line>` element, relative to the @sOffset  given in the
    /// `<roadMark>` element
    pub s_offset: Length,
    /// Lateral offset from the lane border. If `<sway>` element is present, the lateral offset
    /// follows the sway.
    pub t_offset: Length,
    /// Line width. This attribute supersedes the definition in the `<roadMark>` element.
    pub width: Option<Length>,
}

impl ExplicitLine {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "length" => Some(self.length.value.to_scientific_string()).as_deref(),
            "rule" => self.rule.as_ref().map(Rule::as_str),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for ExplicitLine
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            length: read.attribute("length").map(Length::new::<meter>)?,
            rule: read.attribute_opt("rule")?,
            s_offset: read.attribute("sOffset").map(Length::new::<meter>)?,
            t_offset: read.attribute("tOffset").map(Length::new::<meter>)?,
            width: read.attribute_opt("width")?.map(Length::new::<meter>),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for ExplicitLine {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            length: Length::new::<meter>(u.not_nan_f64()?),
            rule: u.arbitrary()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            t_offset: Length::new::<meter>(u.not_nan_f64()?),
            width: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
        })
    }
}
