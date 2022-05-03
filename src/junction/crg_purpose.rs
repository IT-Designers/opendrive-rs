#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum CrgPurpose {
    Elevation,
    Friction,
}

impl_from_str_as_str!(
    CrgPurpose,
    "elevation" => Elevation,
    "friction" => Friction,
);
