use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Lane height shall be defined along the h-coordinate. Lane height may be used to elevate a lane
/// independent from the road elevation. Lane height is used to implement small-scale elevation such
/// as raising pedestrian walkways. Lane height is specified as offset from the road (including
/// elevation, superelevation, shape) in z direction.
#[derive(Debug, Clone, PartialEq)]
pub struct Height {
    /// Inner offset from road level
    pub inner: Length,
    /// Outer offset from road level
    pub outer: Length,
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    pub s_offset: Length,
}

impl Height {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            inner: find_map_parse_attr!(attributes, "inner", f64).map(Length::new::<meter>)?,
            outer: find_map_parse_attr!(attributes, "outer", f64).map(Length::new::<meter>)?,
            s_offset: find_map_parse_attr!(attributes, "sOffset", f64).map(Length::new::<meter>)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "inner" => Some(self.inner.value.to_scientific_string()).as_deref(),
            "outer" => Some(self.outer.value.to_scientific_string()).as_deref(),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
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
impl arbitrary::Arbitrary<'_> for Height {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            inner: Length::new::<meter>(u.not_nan_f64()?),
            outer: Length::new::<meter>(u.not_nan_f64()?),
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}
