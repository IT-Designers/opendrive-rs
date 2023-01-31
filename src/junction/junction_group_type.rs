#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
