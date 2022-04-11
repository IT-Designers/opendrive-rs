use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Defines the maximum allowed speed on a given lane. Each element is valid in direction of the
/// increasing s-coordinate until a new element is defined.
#[derive(Debug, Clone, PartialEq)]
pub struct Speed {
    /// Maximum allowed speed. If the attribute unit is not specified, m/s is used as default.
    pub max: f64,
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    pub s_offset: Length,
    /// Unit of the attribute max
    pub unit: Option<SpeedUnit>,
}

impl Speed {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            max: find_map_parse_attr!(attributes, "max", f64)?,
            s_offset: find_map_parse_attr!(attributes, "sOffset", f64).map(Length::new::<meter>)?,
            unit: find_map_parse_attr!(attributes, "unit", Option<SpeedUnit>)?,
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
            "max" => Some(self.max.to_scientific_string()).as_deref(),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
            "unit" => self.unit.as_ref().map(SpeedUnit::as_str),
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
impl arbitrary::Arbitrary<'_> for Speed {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            max: u.not_nan_f64()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            unit: u.arbitrary()?,
        })
    }
}

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
