#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum AccessRule {
    Allow,
    Deny,
}

impl_from_str_as_str!(
    AccessRule,
    "allow" => Allow,
    "deny" => Deny,
);
