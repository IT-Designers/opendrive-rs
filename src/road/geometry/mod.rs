use crate::core::additional_data::AdditionalData;
use arc::Arc;
use geometry_type::GeometryType;
use line::Line;
use param_poly_3::ParamPoly3;
use poly_3::Poly3;
use spiral::Spiral;
use std::borrow::Cow;
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length};
use uom::si::length::meter;

pub mod arc;
pub mod geometry_type;
pub mod line;
pub mod param_poly_3;
pub mod param_poly_3_p_range;
pub mod plan_view;
pub mod poly_3;
pub mod spiral;

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
    pub additional_data: AdditionalData,
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
        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Geometry
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut choice = None;
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "line" => Line => |v| choice = Some(GeometryType::Line(v)),
            "spiral" => Spiral => |v| choice = Some(GeometryType::Spiral(v)),
            "arc" => Arc => |v| choice = Some(GeometryType::Arc(v)),
            "poly3" => Poly3 => |v| choice = Some(GeometryType::Poly3(v)),
            "paramPoly3" => ParamPoly3 => |v| choice = Some(GeometryType::ParamPoly3(v)),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            hdg: read.attribute("hdg").map(Angle::new::<radian>)?,
            length: read.attribute("length").map(Length::new::<meter>)?,
            s: read.attribute("s").map(Length::new::<meter>)?,
            x: read.attribute("x").map(Length::new::<meter>)?,
            y: read.attribute("y").map(Length::new::<meter>)?,
            choice: choice.ok_or_else(|| {
                crate::parser::Error::missing_element(
                    read.path().to_string(),
                    "line|spiral|arc|poly3|paramPoly3",
                    core::any::type_name::<GeometryType>(),
                )
            })?,
            additional_data,
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
            additional_data: u.arbitrary()?,
        })
    }
}
