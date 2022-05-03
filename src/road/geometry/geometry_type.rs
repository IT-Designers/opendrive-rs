use crate::road::geometry::arc::Arc;
use crate::road::geometry::line::Line;
use crate::road::geometry::param_poly_3::ParamPoly3;
use crate::road::geometry::poly_3::Poly3;
use crate::road::geometry::spiral::Spiral;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum GeometryType {
    Line(Line),
    Spiral(Spiral),
    Arc(Arc),
    Poly3(Poly3),
    ParamPoly3(ParamPoly3),
}
