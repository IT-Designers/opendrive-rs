use serde_derive::{Deserialize, Serialize};

/// Defines the characteristics of the road elevation along the reference line.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElevationProfile {
    pub elevation: Elevation,
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

/// Contains a series of superelevation elements that define the characteristics of the road
/// surface's banking along the reference line.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LateralProfile {
    #[serde(rename = "superelevation", default = "Vec::new")]
    pub super_elevation: Vec<SuperElevation>,
    #[serde(default = "Vec::new")]
    pub shape: Vec<Shape>,
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
