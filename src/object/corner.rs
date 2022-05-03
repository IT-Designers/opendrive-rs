use crate::object::corner_local::CornerLocal;
use crate::object::corner_road::CornerRoad;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Corner {
    Road(CornerRoad),
    Local(CornerLocal),
}
