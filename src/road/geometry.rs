use serde_aux::field_attributes::deserialize_number_from_string;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use uom::si::f64::{Angle, Curvature, Length};

/// Contains geometry elements that define the layout of the road reference line in the x/y-plane
/// (plan view).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanView {
    pub geometry: Vec<Geometry>,
    #[serde(flatten)]
    pub additional_data: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    #[serde(flatten)]
    pub choice: GeometryType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GeometryType {
    #[serde(rename = "line")]
    Line(Line),
    #[serde(rename = "spiral")]
    Spiral(Spiral),
    #[serde(rename = "arc")]
    Arc(Arc),
    #[serde(rename = "poly3")]
    Poly3(Poly3),
    #[serde(rename = "paramPoly3")]
    ParamPoly3(ParamPoly3),
}

/// A straight line is the simplest geometry element. It contains no further attributes.
/// In ASAM OpenDRIVE, a straight line is represented by a `<line>` element within the `<geometry>`
/// element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Line {
    // lol
}

/// In ASAM OpenDRIVE, a spiral is represented by a `<spiral>` element within the `<geometry>`
/// element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Spiral {
    /// Curvature at the start of the element
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "curvStart")]
    pub curvature_start: Curvature,
    /// Curvature at the end of the element
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "curvEnd")]
    pub curvature_end: Curvature,
}

/// An arc describes a road reference line with constant curvature. In ASAM OpenDRIVE, an arc is
/// represented by an `<arc>` element within the `<geometry>` element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Arc {
    /// Constant curvature throughout the element
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub curvature: Curvature,
}

/// In ASAM OpenDRIVE, a cubic polynom is represented by a `<poly3>` element within the `<geometry>`
/// element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Poly3 {
    /// Polynom parameter a
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub a: f64,
    /// Polynom parameter b
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub b: f64,
    /// Polynom parameter c
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub c: f64,
    /// Polynom parameter d
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub d: f64,
}

/// In ASAM OpenDRIVE, parametric cubic curves are represented by `<paramPoly3>` elements within the
/// `<geometry>` element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParamPoly3 {
    /// Polynom parameter a for u
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "aU")]
    pub a_u: f64,
    /// Polynom parameter a for v
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "aV")]
    pub a_v: f64,
    /// Polynom parameter b for u
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "bU")]
    pub b_u: f64,
    /// Polynom parameter b for v
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "bV")]
    pub b_v: f64,
    /// Polynom parameter c for u
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "cU")]
    pub c_u: f64,
    /// Polynom parameter c for v
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "cV")]
    pub c_v: f64,
    /// Polynom parameter d for u
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "dU")]
    pub d_u: f64,
    /// Polynom parameter d for v
    // https://github.com/RReverser/serde-xml-rs/issues/137
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "dV")]
    pub d_v: f64,
    /// Range of parameter p.
    ///   * Case arcLength: p in [0, @length of `<geometry>`]
    ///   * Case normalized: p in [0, 1]
    #[serde(rename = "pRange")]
    pub p_range: f64,
}
