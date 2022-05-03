#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum BridgeType {
    Concrete,
    Steel,
    Brick,
    Wood,
}

impl_from_str_as_str!(
    BridgeType,
    "concrete" => Concrete,
    "steel" => Steel,
    "brick" => Brick,
    "wood" => Wood,
);
