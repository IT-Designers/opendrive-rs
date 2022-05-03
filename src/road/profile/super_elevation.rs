use std::borrow::Cow;

/// Defined as the road section’s roll angle around the s-axis. Elements must be defined in
/// ascending order along the reference line. The parameters of an element are valid until the next
/// element starts or the road reference line ends. Per default, the superelevation of a road is
/// zero.
#[derive(Debug, Clone, PartialEq)]
pub struct SuperElevation {
    /// Polynom parameter a, superelevation at @s (ds=0)
    pub a: f64,
    /// Polynom parameter b
    pub b: f64,
    /// Polynom parameter c
    pub c: f64,
    /// Polynom parameter d
    pub d: f64,
    /// s-coordinate of start position
    pub s: f64,
}

impl SuperElevation {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "a" => &self.a.to_scientific_string(),
            "b" => &self.b.to_scientific_string(),
            "c" => &self.c.to_scientific_string(),
            "d" => &self.d.to_scientific_string(),
            "s" => &self.s.to_scientific_string(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for SuperElevation
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            a: read.attribute("a")?,
            b: read.attribute("b")?,
            c: read.attribute("c")?,
            d: read.attribute("d")?,
            s: read.attribute("s")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for SuperElevation {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            a: u.not_nan_f64()?,
            b: u.not_nan_f64()?,
            c: u.not_nan_f64()?,
            d: u.not_nan_f64()?,
            s: u.not_nan_f64()?,
        })
    }
}
