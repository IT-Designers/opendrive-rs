use std::borrow::Cow;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Defines the characteristics of the road elevation along the reference line.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct ElevationProfile {
    pub elevation: Vec<Elevation>,
}

impl ElevationProfile {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut elevation = Vec::new();

        find_map_parse_elem!(
            events,
            "elevation" => |attributes| {
                elevation.push(Elevation::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self { elevation })
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
        for elevation in &self.elevation {
            visit_children!(visitor, "elevation" => elevation);
        }
        Ok(())
    }
}

/// Defines an elevation element at a given position on the reference line. Elements shall be
/// defined in ascending order along the reference line. The s length does not change with the
/// elevation.
#[derive(Debug, Clone, PartialEq)]
pub struct Elevation {
    /// Polynom parameter a, elevation at @s (ds=0)
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

impl Elevation {
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
            s: find_map_parse_attr!(attributes, "s", f64)?,
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

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Elevation {
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

/// Contains a series of superelevation elements that define the characteristics of the road
/// surface's banking along the reference line.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct LateralProfile {
    pub super_elevation: Vec<SuperElevation>,
    pub shape: Vec<Shape>,
}

impl LateralProfile {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut super_elevation = Vec::new();
        let mut shape = Vec::new();

        find_map_parse_elem!(
            events,
            "superelevation" => |attributes| {
                super_elevation.push(SuperElevation::from_events(events, attributes)?);
                Ok(())
            },
            "shape" => |attributes| {
                shape.push(Shape::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self {
            super_elevation,
            shape,
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
        for elevation in &self.super_elevation {
            visit_children!(visitor, "superelevation" => elevation);
        }

        for shape in &self.shape {
            visit_children!(visitor, "shape" => shape);
        }

        Ok(())
    }
}

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
            s: find_map_parse_attr!(attributes, "s", f64)?,
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

/// Defined as the road section’s surface relative to the reference plane. There may be several
/// shape definitions at one s-position that have different t-values, thereby describing the curvy
/// shape of the road.
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    /// Polynom parameter a, relative height at @t (dt=0)
    pub a: f64,
    /// Polynom parameter b
    pub b: f64,
    /// Polynom parameter c
    pub c: f64,
    /// Polynom parameter d
    pub d: f64,
    /// s-coordinate of start position
    pub s: f64,
    /// t-coordinate of start position
    pub t: f64,
}

impl Shape {
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
            s: find_map_parse_attr!(attributes, "s", f64)?,
            t: find_map_parse_attr!(attributes, "t", f64)?,
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
            "s" => &self.s.to_scientific_string(),
            "t" => &self.t.to_scientific_string(),
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
impl arbitrary::Arbitrary<'_> for Shape {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            a: u.not_nan_f64()?,
            b: u.not_nan_f64()?,
            c: u.not_nan_f64()?,
            d: u.not_nan_f64()?,
            s: u.not_nan_f64()?,
            t: u.not_nan_f64()?,
        })
    }
}
