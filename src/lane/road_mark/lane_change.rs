#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
