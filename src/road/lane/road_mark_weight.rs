#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum RoadMarkWeight {
    Standard,
    Bold,
}

impl_from_str_as_str!(
    RoadMarkWeight,
    "standard" => Standard,
    "bold" => Bold,
);
