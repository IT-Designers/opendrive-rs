#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Weight {
    Standard,
    Bold,
}

impl_from_str_as_str!(
    Weight,
    "standard" => Standard,
    "bold" => Bold,
);
