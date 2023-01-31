#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum CrgMode {
    Global,
}

impl_from_str_as_str!(
    CrgMode,
    "global" => Global,
);
