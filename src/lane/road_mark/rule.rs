#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Rule {
    NoPassing,
    Caution,
    None,
}

impl_from_str_as_str!(
    Rule,
    "no passing" => NoPassing,
    "caution" => Caution,
    "none" => None,
);
