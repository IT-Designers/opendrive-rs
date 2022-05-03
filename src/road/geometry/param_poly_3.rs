use crate::road::geometry::param_poly_3_p_range::ParamPoly3pRange;
use std::borrow::Cow;

/// In ASAM OpenDRIVE, parametric cubic curves are represented by `<paramPoly3>` elements within the
/// `<geometry>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct ParamPoly3 {
    /// Polynom parameter a for u
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub a_u: f64,
    /// Polynom parameter a for v
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub a_v: f64,
    /// Polynom parameter b for u
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub b_u: f64,
    /// Polynom parameter b for v
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub b_v: f64,
    /// Polynom parameter c for u
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub c_u: f64,
    /// Polynom parameter c for v
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub c_v: f64,
    /// Polynom parameter d for u
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub d_u: f64,
    /// Polynom parameter d for v
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub d_v: f64,
    /// Range of parameter p.
    ///   * Case arcLength: p in [0, @length of `<geometry>`]
    ///   * Case normalized: p in [0, 1]
    pub p_range: ParamPoly3pRange,
}

impl ParamPoly3 {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "aU" => &self.a_u.to_scientific_string(),
            "aV" => &self.a_v.to_scientific_string(),
            "bU" => &self.b_u.to_scientific_string(),
            "bV" => &self.b_v.to_scientific_string(),
            "cU" => &self.c_u.to_scientific_string(),
            "cV" => &self.c_v.to_scientific_string(),
            "dU" => &self.d_u.to_scientific_string(),
            "dV" => &self.d_v.to_scientific_string(),
            "pRange" => self.p_range.as_str(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for ParamPoly3
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            a_u: read.attribute("aU")?,
            a_v: read.attribute("aV")?,
            b_u: read.attribute("bU")?,
            b_v: read.attribute("bV")?,
            c_u: read.attribute("cU")?,
            c_v: read.attribute("cV")?,
            d_u: read.attribute("dU")?,
            d_v: read.attribute("dV")?,
            p_range: read.attribute("pRange")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for ParamPoly3 {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            a_u: u.not_nan_f64()?,
            a_v: u.not_nan_f64()?,
            b_u: u.not_nan_f64()?,
            b_v: u.not_nan_f64()?,
            c_u: u.not_nan_f64()?,
            c_v: u.not_nan_f64()?,
            d_u: u.not_nan_f64()?,
            d_v: u.not_nan_f64()?,
            p_range: u.arbitrary()?,
        })
    }
}
