#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Unit {
    Distance(DistanceUnit),
    Speed(SpeedUnit),
    Mass(MassUnit),
    Slope(SlopeUnit),
}

impl Unit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Distance(v) => v.as_str(),
            Self::Speed(v) => v.as_str(),
            Self::Mass(v) => v.as_str(),
            Self::Slope(v) => v.as_str(),
        }
    }
}

impl core::str::FromStr for Unit {
    type Err = crate::parser::InvalidEnumValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(distance) = DistanceUnit::from_str(s) {
            Ok(Self::Distance(distance))
        } else if let Ok(speed) = SpeedUnit::from_str(s) {
            Ok(Self::Speed(speed))
        } else if let Ok(mass) = MassUnit::from_str(s) {
            Ok(Self::Mass(mass))
        } else if let Ok(slope) = SlopeUnit::from_str(s) {
            Ok(Self::Slope(slope))
        } else {
            Err(crate::parser::InvalidEnumValue {
                r#type: core::any::type_name::<Self>().to_string(),
                value: s.to_string(),
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum DistanceUnit {
    Meter,
    KiloMeter,
    Feet,
    Mile,
}

impl_from_str_as_str!(
    DistanceUnit,
    "m" => Meter,
    "km" => KiloMeter,
    "ft" => Feet,
    "mile" => Mile,
);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum SpeedUnit {
    KilometersPerHour,
    MetersPerSecond,
    MilesPerHour,
}

impl_from_str_as_str!(
    SpeedUnit,
    "km/h" => KilometersPerHour,
    "m/s" => MetersPerSecond,
    "mph" => MilesPerHour,
);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum MassUnit {
    KiloGram,
    Ton,
}

impl_from_str_as_str!(
    MassUnit,
    "kg" => KiloGram,
    "t" => Ton,
);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum SlopeUnit {
    Percentage,
}

impl_from_str_as_str!(
    SlopeUnit,
    "%" => Percentage,
);
