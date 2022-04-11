use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Used to add rules that are not covered by any of the other lane attributes that are described in
/// this specification.
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    pub s_offset: Length,
    /// Free text; currently recommended values are
    /// - "no stopping at any time"
    /// - "disabled parking"
    /// - "car pool"
    pub value: String,
}

impl Rule {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            s_offset: find_map_parse_attr!(attributes, "sOffset", f64).map(Length::new::<meter>)?,
            value: find_map_parse_attr!(attributes, "value", String)?,
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
            "sOffset" => &self.s_offset.value.to_scientific_string(),
            "value" => &self.value,
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

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Rule {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            value: u.arbitrary()?,
        })
    }
}