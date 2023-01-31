#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum SideType {
    Left,
    Right,
    Front,
    Rear,
}

impl_from_str_as_str!(
    SideType,
    "left" => Left,
    "right" => Right,
    "front" => Front,
    "rear" => Rear,
);
