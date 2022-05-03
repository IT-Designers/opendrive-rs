#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum StationType {
    Small,
    Medium,
    Large,
}

impl_from_str_as_str!(
    StationType,
    "small" => Small,
    "medium" => Medium,
    "large" => Large,
);
