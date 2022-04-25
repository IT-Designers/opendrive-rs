use std::borrow::Cow;
use uom::si::angle::radian;
use uom::si::curvature::radian_per_meter;
use uom::si::f64::{Angle, Curvature, Length};
use uom::si::length::meter;
use vec1::Vec1;

/// Contains geometry elements that define the layout of the road reference line in the x/y-plane
/// (plan view).
#[derive(Debug, Clone, PartialEq)]
pub struct PlanView {
    pub geometry: Vec1<Geometry>,
    // TODO pub additional_data: HashMap<String, String>,
}

impl PlanView {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(visitor)
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for geometry in &self.geometry {
            visit_children!(visitor, "geometry" => geometry);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for PlanView
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut geometry = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "geometry" true => Geometry => |v| geometry.push(v),
        );

        Ok(Self {
            geometry: Vec1::try_from_vec(geometry).unwrap(),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for PlanView {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            geometry: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    /// Start orientation (inertial heading)
    pub hdg: Angle,
    /// Length of the element's reference line
    pub length: Length,
    /// s-coordinate of start position
    pub s: Length,
    /// Start position (x inertial)
    pub x: Length,
    /// Start position (y inertial)
    pub y: Length,
    pub choice: GeometryType,
}

impl Geometry {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "hdg" => &self.hdg.value.to_scientific_string(),
            "length" => &self.length.value.to_scientific_string(),
            "s" => &self.s.value.to_scientific_string(),
            "x" => &self.x.value.to_scientific_string(),
            "y" => &self.y.value.to_scientific_string(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        match &self.choice {
            GeometryType::Line(value) => visit_children!(visitor, "line" => value),
            GeometryType::Spiral(value) => visit_children!(visitor, "spiral" => value),
            GeometryType::Arc(value) => visit_children!(visitor, "arc" => value),
            GeometryType::Poly3(value) => visit_children!(visitor, "poly3" => value),
            GeometryType::ParamPoly3(value) => {
                visit_children!(visitor, "paramPoly3" => value)
            }
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Geometry
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut choice = None;

        match_child_eq_ignore_ascii_case!(
            read,
            "line" => Line => |v| choice = Some(GeometryType::Line(v)),
            "spiral" => Spiral => |v| choice = Some(GeometryType::Spiral(v)),
            "arc" => Arc => |v| choice = Some(GeometryType::Arc(v)),
            "poly3" => Poly3 => |v| choice = Some(GeometryType::Poly3(v)),
            "paramPoly3" => ParamPoly3 => |v| choice = Some(GeometryType::ParamPoly3(v)),
        );

        Ok(Self {
            hdg: read.attribute("hdg").map(Angle::new::<radian>)?,
            length: read.attribute("length").map(Length::new::<meter>)?,
            s: read.attribute("s").map(Length::new::<meter>)?,
            x: read.attribute("x").map(Length::new::<meter>)?,
            y: read.attribute("y").map(Length::new::<meter>)?,
            choice: choice.ok_or_else(|| {
                crate::parser::Error::missing_element(
                    read.path.to_string(),
                    "line|spiral|arc|poly3|paramPoly3",
                    core::any::type_name::<GeometryType>(),
                )
            })?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Geometry {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            hdg: Angle::new::<radian>(u.not_nan_f64()?),
            length: Length::new::<meter>(u.not_nan_f64()?),
            s: Length::new::<meter>(u.not_nan_f64()?),
            x: Length::new::<meter>(u.not_nan_f64()?),
            y: Length::new::<meter>(u.not_nan_f64()?),
            choice: u.arbitrary()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum GeometryType {
    Line(Line),
    Spiral(Spiral),
    Arc(Arc),
    Poly3(Poly3),
    ParamPoly3(ParamPoly3),
}

/// A straight line is the simplest geometry element. It contains no further attributes.
/// In ASAM OpenDRIVE, a straight line is represented by a `<line>` element within the `<geometry>`
/// element.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Line {
    // lol
}

impl Line {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(visitor)
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(visitor);
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Line
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(_read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

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

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
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
        Ok(Self {
            curvature_start: Curvature::new::<radian_per_meter>(u.not_nan_f64()?),
            curvature_end: Curvature::new::<radian_per_meter>(u.not_nan_f64()?),
        })
    }
}

/// An arc describes a road reference line with constant curvature. In ASAM OpenDRIVE, an arc is
/// represented by an `<arc>` element within the `<geometry>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Arc {
    /// Constant curvature throughout the element
    // https://github.com/RReverser/serde-xml-rs/issues/137
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

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
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

/// In ASAM OpenDRIVE, a cubic polynom is represented by a `<poly3>` element within the `<geometry>`
/// element.
#[derive(Debug, Clone, PartialEq)]
pub struct Poly3 {
    /// Polynom parameter a
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub a: f64,
    /// Polynom parameter b
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub b: f64,
    /// Polynom parameter c
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub c: f64,
    /// Polynom parameter d
    // https://github.com/RReverser/serde-xml-rs/issues/137
    pub d: f64,
}

impl Poly3 {
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Poly3
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            a: read.attribute("a")?,
            b: read.attribute("b")?,
            c: read.attribute("c")?,
            d: read.attribute("d")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Poly3 {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            a: u.not_nan_f64()?,
            b: u.not_nan_f64()?,
            c: u.not_nan_f64()?,
            d: u.not_nan_f64()?,
        })
    }
}

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
    pub p_range: f64,
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
            "pRange" => &self.p_range.to_scientific_string(),
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

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
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
            p_range: u.not_nan_f64()?,
        })
    }
}
