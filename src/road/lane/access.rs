use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Defines access restrictions for certain types of road users.
/// Each element is valid in direction of the increasing s coordinate until a new element is
/// defined. If multiple elements are defined, they shall be listed in ascending order.
#[derive(Debug, Clone, PartialEq)]
pub struct Access {
    /// Identifier of the participant to whom the restriction applies
    pub restriction: RestrictionType,
    /// Specifies whether the participant given in the attribute @restriction is allowed or denied
    /// access to the given lane
    pub rule: Option<AccessRule>,
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    pub s_offset: Length,
}

impl Access {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            restriction: find_map_parse_attr!(attributes, "restriction", RestrictionType)?,
            rule: find_map_parse_attr!(attributes, "rule", Option<AccessRule>)?,
            s_offset: find_map_parse_attr!(attributes, "sOffset", f64).map(Length::new::<meter>)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "restriction" => Some(self.restriction.as_str()),
            "rule" => self.rule.as_ref().map(AccessRule::as_str),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(visitor);
        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Access {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            restriction: u.arbitrary()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            rule: u.arbitrary()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum RestrictionType {
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
    RestrictionType,
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
