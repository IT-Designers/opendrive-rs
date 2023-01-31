#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
