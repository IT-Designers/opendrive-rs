#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Source {
    Sensor,
    Cadaster,
    Custom,
}

impl_from_str_as_str!(
    Source,
    "sensor" => Sensor,
    "cadaster" => Cadaster,
    "custom" => Custom,
);
