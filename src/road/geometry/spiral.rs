use std::borrow::Cow;
use uom::si::curvature::radian_per_meter;
use uom::si::f64::Curvature;

/// In ASAM OpenDRIVE, a spiral is represented by a `<spiral>` element within the `<geometry>`
/// element.
#[derive(Debug, Clone, PartialEq)]
pub struct Spiral {
    /// Curvature at the start of the element
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub curvature_start: Curvature,
    /// Curvature at the end of the element
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub curvature_end: Curvature,
}

impl Spiral {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "curvStart" => &self.curvature_start.value.to_scientific_string(),
            "curvEnd" => &self.curvature_end.value.to_scientific_string(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Spiral
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            curvature_start: read
                .attribute("curvStart")
                .map(Curvature::new::<radian_per_meter>)?,
            curvature_end: read
                .attribute("curvEnd")
                .map(Curvature::new::<radian_per_meter>)?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Spiral {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        use uom::si::curvature::radian_per_meter;
        Ok(Self {
            curvature_start: Curvature::new::<radian_per_meter>(u.not_nan_f64()?),
            curvature_end: Curvature::new::<radian_per_meter>(u.not_nan_f64()?),
        })
    }
}
