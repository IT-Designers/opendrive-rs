use serde_derive::{Deserialize, Serialize};
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Defines the characteristics of the road elevation along the reference line.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
}

/// Defines an elevation element at a given position on the reference line. Elements shall be
/// defined in ascending order along the reference line. The s length does not change with the
/// elevation.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
}

/// Contains a series of superelevation elements that define the characteristics of the road
/// surface's banking along the reference line.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LateralProfile {
    #[serde(rename = "superelevation", default = "Vec::new")]
    pub super_elevation: Vec<SuperElevation>,
    #[serde(default = "Vec::new")]
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
}

/// Defined as the road section’s roll angle around the s-axis. Elements must be defined in
/// ascending order along the reference line. The parameters of an element are valid until the next
/// element starts or the road reference line ends. Per default, the superelevation of a road is
/// zero.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
}

/// Defined as the road section’s surface relative to the reference plane. There may be several
/// shape definitions at one s-position that have different t-values, thereby describing the curvy
/// shape of the road.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
}
