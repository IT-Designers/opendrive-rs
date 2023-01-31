pub use allow_deprecated::ObjectType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Orientation {
    Plus,
    Minus,
    None,
}

impl_from_str_as_str!(
    Orientation,
    "+" => Plus,
    "-" => Minus,
    "none" => None,
);

#[allow(deprecated)]
mod allow_deprecated {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
    pub enum ObjectType {
        /// i.e. unknown
        None,
        /// for anything that is not further categorized
        Obstacle,
        #[deprecated]
        Car,
        Pole,
        Tree,
        Vegetation,
        Barrier,
        Building,
        ParkingSpace,
        Patch,
        Railing,
        TrafficIsland,
        Crosswalk,
        StreetLamp,
        Gantry,
        SoundBarrier,
        #[deprecated]
        Van,
        #[deprecated]
        Bus,
        #[deprecated]
        Trailer,
        #[deprecated]
        Bike,
        #[deprecated]
        Motorbike,
        #[deprecated]
        Tram,
        #[deprecated]
        Train,
        #[deprecated]
        Pedestrian,
        #[deprecated]
        Wind,
        RoadMark,
    }

    impl_from_str_as_str!(
        ObjectType,
        "none" => None,
        "obstacle" => Obstacle,
        "car" => Car,
        "pole" => Pole,
        "tree" => Tree,
        "vegetation" => Vegetation,
        "barrier" => Barrier,
        "building" => Building,
        "parkingSpace" => ParkingSpace,
        "patch" => Patch,
        "railing" => Railing,
        "trafficIsland" => TrafficIsland,
        "crosswalk" => Crosswalk,
        "streetLamp" => StreetLamp,
        "gantry" => Gantry,
        "soundBarrier" => SoundBarrier,
        "van" => Van,
        "bus" => Bus,
        "trailer" => Trailer,
        "bike" => Bike,
        "motorbike" => Motorbike,
        "tram" => Tram,
        "train" => Train,
        "pedestrian" => Pedestrian,
        "wind" => Wind,
        "roadMark" => RoadMark,
    );
}
