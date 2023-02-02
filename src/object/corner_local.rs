use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Used to describe complex forms of objects. Defines a corner point on the object outline relative
/// to the object pivot point in local u/v-coordinates. The insertion point and the orientation of
/// the object are given by the @s, @t, @zOffset and @hdg attributes of the  element.
#[derive(Debug, Clone, PartialEq)]
pub struct CornerLocal {
    /// Height of the object at this corner, along the z-axis
    pub height: Length,
    /// ID of the outline point. Shall be unique within one outline.
    pub id: Option<u64>,
    /// Local u-coordinate of the corner
    pub u: Length,
    /// Local v-coordinate of the corner
    pub v: Length,
    /// Local z-coordinate of the corner
    pub z: Length,
}

impl CornerLocal {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "height" => Some(self.height.value.to_scientific_string()).as_deref(),
            "id" => self.id.map(|v| v.to_string()).as_deref(),
            "u" => Some(self.u.value.to_scientific_string()).as_deref(),
            "v" => Some(self.v.value.to_scientific_string()).as_deref(),
            "z" => Some(self.z.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for CornerLocal
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            height: read.attribute("height").map(Length::new::<meter>)?,
            id: read.attribute_opt("id")?,
            u: read.attribute("u").map(Length::new::<meter>)?,
            v: read.attribute("v").map(Length::new::<meter>)?,
            z: read.attribute("z").map(Length::new::<meter>)?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for CornerLocal {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            height: Length::new::<meter>(u.not_nan_f64()?),
            id: u.arbitrary()?,
            u: Length::new::<meter>(u.not_nan_f64()?),
            v: Length::new::<meter>(u.not_nan_f64()?),
            z: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}
