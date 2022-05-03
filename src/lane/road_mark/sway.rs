use std::borrow::Cow;

/// Relocates the lateral reference position for the following (explicit) type definition and thus
/// defines an offset. The sway offset is relative to the nominal reference position of the lane
/// marking, meaning the lane border.
#[derive(Debug, Clone, PartialEq)]
pub struct Sway {
    /// Polynom parameter a, sway value at @s (ds=0)
    a: f64,
    /// Polynom parameter b
    b: f64,
    /// Polynom parameter c
    c: f64,
    /// Polynom parameter d
    d: f64,
    /// s-coordinate of start position of the `<sway>` element, relative to the @sOffset given in
    /// the `<roadMark>` element
    d_s: f64,
}

impl Sway {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "a" => Some(self.a.to_scientific_string()).as_deref(),
            "b" => Some(self.b.to_scientific_string()).as_deref(),
            "c" => Some(self.c.to_scientific_string()).as_deref(),
            "d" => Some(self.d.to_scientific_string()).as_deref(),
            "d_s" => Some(self.d_s.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Sway
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
            d_s: read.attribute("d_s")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Sway {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            a: u.not_nan_f64()?,
            b: u.not_nan_f64()?,
            c: u.not_nan_f64()?,
            d: u.not_nan_f64()?,
            d_s: u.not_nan_f64()?,
        })
    }
}
