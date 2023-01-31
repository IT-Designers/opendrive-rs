#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum OutlineFillType {
    Grass,
    Concrete,
    Cobble,
    Asphalt,
    Pavement,
    Gravel,
    Soil,
}

impl_from_str_as_str!(
    OutlineFillType,
    "grass" => Grass,
    "concrete" => Concrete,
    "cobble" => Cobble,
    "asphalt" => Asphalt,
    "pavement" => Pavement,
    "gravel" => Gravel,
    "soil" => Soil,
);
