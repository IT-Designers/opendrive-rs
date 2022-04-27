#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum SwitchPosition {
    Dynamic,
    Straight,
    Turn,
}

impl_from_str_as_str!(
    SwitchPosition,
    "dynamic" => Dynamic,
    "straight" => Straight,
    "turn" => Turn,
);
