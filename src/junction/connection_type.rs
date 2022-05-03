#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum ConnectionType {
    Default,
    Virtual,
}

impl_from_str_as_str!(
    ConnectionType,
    "default" => Default,
    "virtual" => Virtual,
);
