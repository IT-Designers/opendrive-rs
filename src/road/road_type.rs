use crate::core::additional_data::AdditionalData;
use crate::lane::speed::Speed;
use crate::road::country_code::CountryCode;
use crate::road::road_type_e::RoadTypeE;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// A road type element is valid for the entire cross section of a road. It is valid until a new
/// road type element is provided or until the road ends.
#[derive(Debug, Clone, PartialEq)]
pub struct RoadType {
    pub speed: Option<Speed>,
    /// Country code of the road, see ISO 3166-1, alpha-2 codes.
    pub country: Option<CountryCode>,
    /// s-coordinate of start position
    pub s: Length,
    /// Type of the road defined as enumeration
    pub r#type: RoadTypeE,
    pub additional_data: AdditionalData,
}

impl RoadType {
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

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for RoadType
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut speed = None;
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "speed" => Speed => |v| speed = Some(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            speed,
            country: read.attribute_opt("country")?,
            s: read.attribute("s").map(Length::new::<meter>)?,
            r#type: read.attribute("type")?,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for RoadType {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            speed: u.arbitrary()?,
            country: u.arbitrary()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
            r#type: u.arbitrary()?,
            additional_data: u.arbitrary()?,
        })
    }
}
