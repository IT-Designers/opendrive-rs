/// The known keywords for the road type information
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum RoadTypeE {
    Unknown,
    Rural,
    Motorway,
    Town,
    /// In Germany, lowSpeed is equivalent to a 30km/h zone
    LowSpeed,
    Pedestrian,
    Bicycle,
    TownExpressway,
    TownCollector,
    TownArterial,
    TownPrivate,
    TownLocal,
    TownPlayStreet,
}

impl_from_str_as_str!(
    RoadTypeE,
    "unknown" => Unknown,
    "rural" => Rural,
    "motorway" => Motorway,
    "town" => Town,
    "lowSpeed" => LowSpeed,
    "pedestrian" => Pedestrian,
    "bicycle" => Bicycle,
    "townExpressway" => TownExpressway,
    "townCollector" => TownCollector,
    "townArterial" => TownArterial,
    "townPrivate" => TownPrivate,
    "townLocal" => TownLocal,
    "townPlayStreet" => TownPlayStreet,
);
