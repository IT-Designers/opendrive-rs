use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// To avoid lengthy XML code, objects of the same type may be repeated. Attributes of the repeated
/// object shall overrule the attributes from the original object. If attributes are omitted in the
/// repeated objects, the attributes from the original object apply.
#[derive(Debug, Clone, PartialEq)]
pub struct Repeat {
    /// Distance between two instances of the object;
    /// If this value is zero, then the object is treated like a continuous feature, for example, a
    /// guard rail, a wall, etc.
    pub distance: Length,
    /// Height of the object at @s + @length
    pub height_end: Length,
    /// Height of the object at @s
    pub height_start: Length,
    /// Length of the repeat area, along the reference line in s-direction.
    pub length: Length,
    /// Length of the object at @s + @length
    pub length_end: Option<Length>,
    /// Length of the object at @s
    pub length_start: Option<Length>,
    /// Radius of the object at @s + @length
    pub radius_end: Option<Length>,
    /// Radius of the object at @s
    pub radius_start: Option<Length>,
    /// s-coordinate of start position, overrides the corresponding argument in the original
    /// `<object>` record
    pub s: Length,
    /// Lateral offset of object's reference point at @s + @length
    pub t_end: Length,
    /// Lateral offset of objects reference point at @s
    pub t_start: Length,
    /// Width of the object at @s + @length
    pub width_end: Option<Length>,
    /// Width of the object at @s
    pub width_start: Option<Length>,
    /// z-offset of the object at @s + @length, relative to the elevation of the reference line
    pub z_offset_end: Option<Length>,
    /// z-offset of the object at @s, relative to the elevation of the reference line
    pub z_offset_start: Option<Length>,
}

impl Repeat {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "distance" => Some(self.distance.value.to_scientific_string()).as_deref(),
            "heightEnd" => Some(self.height_end.value.to_scientific_string()).as_deref(),
            "heightStart" => Some(self.height_start.value.to_scientific_string()).as_deref(),
            "length" => Some(self.length.value.to_scientific_string()).as_deref(),
            "lengthEnd" => self.length_end.map(|v| v.value.to_scientific_string()).as_deref(),
            "lengthStart" => self.length_start.map(|v| v.value.to_scientific_string()).as_deref(),
            "radiusEnd" => self.radius_end.map(|v| v.value.to_scientific_string()).as_deref(),
            "radiusStart" => self.radius_start.map(|v| v.value.to_scientific_string()).as_deref(),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "tEnd" => Some(self.t_end.value.to_scientific_string()).as_deref(),
            "tStart" => Some(self.t_start.value.to_scientific_string()).as_deref(),
            "widthEnd" => self.width_end.map(|v| v.value.to_scientific_string()).as_deref(),
            "widthStart" => self.width_start.map(|v| v.value.to_scientific_string()).as_deref(),
            "zOffsetEnd" => self.z_offset_end.map(|v| v.value.to_scientific_string()).as_deref(),
            "zOffsetStart" => self.z_offset_start.map(|v| v.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Repeat
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            distance: read.attribute("distance").map(Length::new::<meter>)?,
            height_end: read.attribute("heightEnd").map(Length::new::<meter>)?,
            height_start: read.attribute("heightStart").map(Length::new::<meter>)?,
            length: read.attribute("length").map(Length::new::<meter>)?,
            length_end: read.attribute_opt("lengthEnd")?.map(Length::new::<meter>),
            length_start: read.attribute_opt("lengthStart")?.map(Length::new::<meter>),
            radius_end: read.attribute_opt("radiusEnd")?.map(Length::new::<meter>),
            radius_start: read.attribute_opt("radiusStart")?.map(Length::new::<meter>),
            s: read.attribute("s").map(Length::new::<meter>)?,
            t_end: read.attribute("tEnd").map(Length::new::<meter>)?,
            t_start: read.attribute("tStart").map(Length::new::<meter>)?,
            width_end: read.attribute_opt("widthEnd")?.map(Length::new::<meter>),
            width_start: read.attribute_opt("widthStart")?.map(Length::new::<meter>),
            z_offset_end: read.attribute_opt("zOffsetEnd")?.map(Length::new::<meter>),
            z_offset_start: read
                .attribute_opt("zOffsetStart")?
                .map(Length::new::<meter>),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Repeat {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            distance: Length::new::<meter>(u.not_nan_f64()?),
            height_end: Length::new::<meter>(u.not_nan_f64()?),
            height_start: Length::new::<meter>(u.not_nan_f64()?),
            length: Length::new::<meter>(u.not_nan_f64()?),
            length_end: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            length_start: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            radius_end: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            radius_start: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            s: Length::new::<meter>(u.not_nan_f64()?),
            t_end: Length::new::<meter>(u.not_nan_f64()?),
            t_start: Length::new::<meter>(u.not_nan_f64()?),
            width_end: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            width_start: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            z_offset_end: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            z_offset_start: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
        })
    }
}
