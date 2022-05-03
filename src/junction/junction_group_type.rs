#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum JunctionGroupType {
    Roundabout,
    Unknown,
}

impl_from_str_as_str!(
    JunctionGroupType,
    "roundabout" => Roundabout,
    "unknown" => Unknown
);
