#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum TunnelType {
    Standard,
    /// i.e. sides are open for daylight
    Underpass,
}

impl_from_str_as_str!(
    TunnelType,
    "standard" => Standard,
    "underpass" => Underpass,
);
