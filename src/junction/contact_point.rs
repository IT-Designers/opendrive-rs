#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum ContactPoint {
    Start,
    End,
}

impl_from_str_as_str!(
    ContactPoint,
    "start" => Start,
    "end" => End,
);
