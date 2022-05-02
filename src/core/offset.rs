use crate::core::additional_data::AdditionalData;
use std::borrow::Cow;
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length};
use uom::si::length::meter;

/// To avoid large coordinates, an offset of the whole dataset may be applied using the `<offset>`
/// element. It enables inertial relocation and re-orientation of datasets. The dataset is first
/// translated by @x, @y, and @z. Afterwards, it is rotated by @hdg around the new origin. Rotation
/// around the z-axis should be avoided. In ASAM OpenDRIVE, the offset of a database is represented
/// by the `<offset>` element within the `<header>` element.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Offset {
    /// Heading offset (rotation around resulting z-axis)
    pub hdg: Angle,
    /// Inertial x offset
    pub x: Length,
    /// Inertial y offset
    pub y: Length,
    /// Inertial z offset
    pub z: Length,
    pub additional_data: AdditionalData,
}

impl Offset {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "hdg" => &self.hdg.value.to_scientific_string(),
            "x" => &self.x.value.to_scientific_string(),
            "y" => &self.y.value.to_scientific_string(),
            "z" => &self.z.value.to_scientific_string(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(visitor);
        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Offset
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut additional_data = AdditionalData::default();

        read.children(|_name, context| additional_data.fill(context))?;

        Ok(Self {
            hdg: read.attribute("hdg").map(Angle::new::<radian>)?,
            x: read.attribute("x").map(Length::new::<meter>)?,
            y: read.attribute("y").map(Length::new::<meter>)?,
            z: read.attribute("z").map(Length::new::<meter>)?,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Offset {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            hdg: Angle::new::<radian>(u.not_nan_f64()?),
            x: Length::new::<meter>(u.not_nan_f64()?),
            y: Length::new::<meter>(u.not_nan_f64()?),
            z: Length::new::<meter>(u.not_nan_f64()?),
            additional_data: u.arbitrary()?,
        })
    }
}
