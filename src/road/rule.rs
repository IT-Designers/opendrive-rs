#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Rule {
    RightHandTraffic,
    LeftHandTraffic,
}

impl_from_str_as_str!(
    Rule,
    "RHT" => RightHandTraffic,
    "LHT" => LeftHandTraffic,
);
