#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum ParamPoly3pRange {
    ArcLength,
    Normalized,
}

impl_from_str_as_str!(
    ParamPoly3pRange,
    "arcLength" => ArcLength,
    "normalized" => Normalized,
);
