#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum JunctionType {
    Default,
    Virtual,
    Direct,
}

impl_from_str_as_str!(
    JunctionType,
    "default" => Default,
    "virtual" => Virtual,
    "direct" => Direct,
);
