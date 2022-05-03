use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Defines a corner point on the object’s outline in road coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct CornerRoad {
    /// dz of the corner relative to road reference line
    pub dz: Length,
    /// Height of the object at this corner, along the z-axis
    pub height: Length,
    /// ID of the outline point. Must be unique within one outline
    pub id: Option<u64>,
    /// s-coordinate of the corner
    pub s: Length,
    /// t-coordinate of the corner
    pub t: Length,
}

impl CornerRoad {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "dz" => Some(self.dz.value.to_scientific_string()).as_deref(),
            "height" => Some(self.height.value.to_scientific_string()).as_deref(),
            "id" => self.id.map(|v| v.to_string()).as_deref(),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "t" => Some(self.t.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for CornerRoad
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            dz: read.attribute("dz").map(Length::new::<meter>)?,
            height: read.attribute("height").map(Length::new::<meter>)?,
            id: read.attribute_opt("id")?,
            s: read.attribute("s").map(Length::new::<meter>)?,
            t: read.attribute("t").map(Length::new::<meter>)?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for CornerRoad {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        use uom::si::f64::Length;
        use uom::si::length::meter;
        Ok(Self {
            dz: Length::new::<meter>(u.not_nan_f64()?),
            height: Length::new::<meter>(u.not_nan_f64()?),
            id: u.arbitrary()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
            t: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}
