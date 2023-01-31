#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Access {
    All,
    Car,
    Women,
    Handicapped,
    Bus,
    Truck,
    Electric,
    Residents,
}

impl_from_str_as_str!(
    Access,
    "all" => All,
    "car" => Car,
    "women" => Women,
    "handicapped" => Handicapped,
    "bus" => Bus,
    "truck" => Truck,
    "electric" => Electric,
    "residents" => Residents,
);
