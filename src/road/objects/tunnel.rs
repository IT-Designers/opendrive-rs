use crate::road::objects::validity::LaneValidity;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Tunnels are modeled as objects in ASAM OpenDRIVE. Tunnels apply to the entire cross section of
/// the road within the given range unless a lane validity element with further restrictions is
/// provided as child element.
#[derive(Debug, Clone, PartialEq)]
pub struct Tunnel {
    /// Degree of daylight intruding the tunnel. Depends on the application.
    pub daylight: Option<f64>,
    /// Unique ID within database
    pub id: String,
    /// Length of the tunnel (in s-direction)
    pub length: Length,
    /// Degree of artificial tunnel lighting. Depends on the application.
    pub lighting: Option<f64>,
    /// Name of the tunnel. May be chosen freely.
    pub name: Option<String>,
    /// s-coordinate
    pub s: Length,
    /// Type of tunnel
    pub r#type: TunnelType,
    pub validity: Vec<LaneValidity>,
}

impl Tunnel {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "daylight" => self.daylight.map(|v| v.to_scientific_string()).as_deref(),
            "id" => Some(self.id.as_str()),
            "length" => Some(self.length.value.to_scientific_string()).as_deref(),
            "lighting" => self.lighting.map(|v| v.to_scientific_string()).as_deref(),
            "name" => self.name.as_deref(),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "type" => Some(self.r#type.as_str()),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for validity in &self.validity {
            visit_children!(visitor, "validity" => validity);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Tunnel
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut validity = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "validity" => LaneValidity => |v| validity.push(v),
        );

        Ok(Self {
            daylight: read.attribute_opt("daylight")?,
            id: read.attribute("id")?,
            length: read.attribute("length").map(Length::new::<meter>)?,
            lighting: read.attribute_opt("lighting")?,
            name: read.attribute_opt("name")?,
            s: read.attribute("s").map(Length::new::<meter>)?,
            r#type: read.attribute("type")?,
            validity,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Tunnel {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            daylight: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?,
            id: u.arbitrary()?,
            length: Length::new::<meter>(u.not_nan_f64()?),
            lighting: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?,
            name: u.arbitrary()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
            r#type: u.arbitrary()?,
            validity: u.arbitrary()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum TunnelType {
    Standard,
    /// i.e. sides are open for daylight
    Underpass,
}

impl_from_str_as_str!(
    TunnelType,
    "standard" => Standard,
    "underpass" => Underpass,
);
