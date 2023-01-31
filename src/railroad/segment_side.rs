#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum SegmentSide {
    Left,
    Right,
}

impl_from_str_as_str!(
    SegmentSide,
    "left" => Left,
    "right" => Right,
);
