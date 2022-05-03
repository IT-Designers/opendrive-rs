use crate::lane::speed::Speed;
use crate::road::country_code::CountryCode;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// A road type element is valid for the entire cross section of a road. It is valid until a new
/// road type element is provided or until the road ends.
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub speed: Option<Speed>,
    /// Country code of the road, see ISO 3166-1, alpha-2 codes.
    pub country: Option<CountryCode>,
    /// s-coordinate of start position
    pub s: Length,
    /// Type of the road defined as enumeration
    pub r#type: RoadType,
}

impl Type {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "country" => self.country.as_ref().map(CountryCode::as_str),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "type" => Some(self.r#type.as_str()),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(speed) = &self.speed {
            visit_children!(visitor, "speed" => speed);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Type
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut speed = None;

        match_child_eq_ignore_ascii_case!(
            read,
            "speed" => Speed => |v| speed = Some(v),
        );

        Ok(Self {
            speed,
            country: read.attribute_opt("country")?,
            s: read.attribute("s").map(Length::new::<meter>)?,
            r#type: read.attribute("type")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Type {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            speed: u.arbitrary()?,
            country: u.arbitrary()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
            r#type: u.arbitrary()?,
        })
    }
}

/// The known keywords for the road type information
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum RoadType {
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
    RoadType,
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
