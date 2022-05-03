#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum LaneChange {
    Increase,
    Decrease,
    Both,
    None,
}

impl_from_str_as_str!(
    LaneChange,
    "increase" => Increase,
    "decrease" => Decrease,
    "both" => Both,
    "none" => None,
);
