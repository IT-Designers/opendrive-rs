use std::borrow::Cow;
use std::collections::HashMap;
use uom::si::angle::radian;
use uom::si::curvature::radian_per_meter;
use uom::si::f64::{Angle, Curvature, Length};
use uom::si::length::meter;
use vec1::Vec1;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Contains geometry elements that define the layout of the road reference line in the x/y-plane
/// (plan view).
#[derive(Debug, Clone, PartialEq)]
pub struct PlanView {
    pub geometry: Vec1<Geometry>,
    // TODO pub additional_data: HashMap<String, String>,
}

impl PlanView {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut geometry = Vec::new();
        let mut additional_data = HashMap::new();

        for attr in attributes {
            additional_data.insert(attr.name.local_name, attr.value);
        }

        find_map_parse_elem!(
            events,
            "geometry" true => |attributes| {
                geometry.push(Geometry::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self {
            geometry: Vec1::try_from_vec(geometry)
                .map_err(|_| crate::parser::Error::child_missing::<Self>())?,
        })
    }

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
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut choice = None;

        find_map_parse_elem!(
            events,
            "line" => |attributes| {
                choice = Some(GeometryType::Line(Line::from_events(events, attributes)?));
                Ok(())
            },
            "spiral" => |attributes| {
                choice = Some(GeometryType::Spiral(Spiral::from_events(events, attributes)?));
                Ok(())
            },
            "arc" => |attributes| {
                choice = Some(GeometryType::Arc(Arc::from_events(events, attributes)?));
                Ok(())
            },
            "poly3" => |attributes| {
                choice = Some(GeometryType::Poly3(Poly3::from_events(events, attributes)?));
                Ok(())
            },
            "paramPoly3" => |attributes| {
                choice = Some(GeometryType::ParamPoly3(ParamPoly3::from_events(events, attributes)?));
                Ok(())
            }
        );

        Ok(Self {
            hdg: find_map_parse_attr!(attributes, "hdg", f64).map(Angle::new::<radian>)?,
            length: find_map_parse_attr!(attributes, "length", f64).map(Length::new::<meter>)?,
            s: find_map_parse_attr!(attributes, "s", f64).map(Length::new::<meter>)?,
            x: find_map_parse_attr!(attributes, "x", f64).map(Length::new::<meter>)?,
            y: find_map_parse_attr!(attributes, "y", f64).map(Length::new::<meter>)?,
            choice: choice.ok_or_else(crate::parser::Error::child_missing::<Self>)?,
        })
    }

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
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {})
    }

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
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            curvature_start: find_map_parse_attr!(attributes, "curvStart", f64)
                .map(Curvature::new::<radian_per_meter>)?,
            curvature_end: find_map_parse_attr!(attributes, "curvEnd", f64)
                .map(Curvature::new::<radian_per_meter>)?,
        })
    }

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
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            curvature: find_map_parse_attr!(attributes, "curvature", f64)
                .map(Curvature::new::<radian_per_meter>)?,
        })
    }

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
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            a: find_map_parse_attr!(attributes, "a", f64)?,
            b: find_map_parse_attr!(attributes, "b", f64)?,
            c: find_map_parse_attr!(attributes, "c", f64)?,
            d: find_map_parse_attr!(attributes, "d", f64)?,
        })
    }

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
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            a_u: find_map_parse_attr!(attributes, "aU", f64)?,
            a_v: find_map_parse_attr!(attributes, "aV", f64)?,
            b_u: find_map_parse_attr!(attributes, "bU", f64)?,
            b_v: find_map_parse_attr!(attributes, "bV", f64)?,
            c_u: find_map_parse_attr!(attributes, "cU", f64)?,
            c_v: find_map_parse_attr!(attributes, "cV", f64)?,
            d_u: find_map_parse_attr!(attributes, "dU", f64)?,
            d_v: find_map_parse_attr!(attributes, "dV", f64)?,
            p_range: find_map_parse_attr!(attributes, "pRange", f64)?,
        })
    }

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
