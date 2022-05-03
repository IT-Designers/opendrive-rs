use std::borrow::Cow;
use uom::si::curvature::radian_per_meter;
use uom::si::f64::Curvature;

/// An arc describes a road reference line with constant curvature. In ASAM OpenDRIVE, an arc is
/// represented by an `<arc>` element within the `<geometry>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Arc {
    /// Constant curvature throughout the element
    pub curvature: Curvature,
}

impl Arc {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "curvature" => &self.curvature.value.to_scientific_string(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Arc
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            curvature: read
                .attribute("curvature")
                .map(Curvature::new::<radian_per_meter>)?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Arc {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            curvature: Curvature::new::<radian_per_meter>(u.not_nan_f64()?),
        })
    }
}
