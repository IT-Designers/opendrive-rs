#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum AccessRestrictionType {
    Simulator,
    AutonomousTraffic,
    Pedestrian,
    PassengerCar,
    Bus,
    Delivery,
    Emergency,
    Taxi,
    ThroughTraffic,
    Truck,
    Bicycle,
    Motorcycle,
    None,
    Trucks,
}

impl_from_str_as_str!(
    AccessRestrictionType,
    "simulator" => Simulator,
    "autonomousTraffic" => AutonomousTraffic,
    "pedestrian" => Pedestrian,
    "passengerCar" => PassengerCar,
    "bus" => Bus,
    "delivery" => Delivery,
    "emergency" => Emergency,
    "taxi" => Taxi,
    "throughTraffic" => ThroughTraffic,
    "truck" => Truck,
    "bicycle" => Bicycle,
    "motorcycle" => Motorcycle,
    "none" => None,
    "trucks" => Trucks,
);
