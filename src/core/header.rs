use crate::core::additional_data::AdditionalData;
use crate::core::geo_reference::GeoReference;
use crate::core::offset::Offset;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::borrow::Cow;
use std::str::FromStr;
use uom::si::f64::Length;
use uom::si::length::meter;

/// The `<header>` element is the very first element within the `<OpenDRIVE>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub rev_major: u16,
    pub rev_minor: u16,
    pub name: Option<String>,
    pub version: Option<String>,
    pub date: Option<String>,
    pub north: Option<Length>,
    pub south: Option<Length>,
    pub east: Option<Length>,
    pub west: Option<Length>,
    pub vendor: Option<String>,
    pub geo_reference: Option<GeoReference>,
    pub offset: Option<Offset>,
    pub additional_data: AdditionalData,
}

impl Header {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "revMajor" => Some(self.rev_major.to_string()).as_deref(),
            "revMinor" => Some(self.rev_minor.to_string()).as_deref(),
            "name" => self.name.as_deref(),
            "version" => self.version.as_deref(),
            "date" => self.date.as_deref(),
            "north" => self.north.map(|v| v.value.to_scientific_string()).as_deref(),
            "south" => self.south.map(|v| v.value.to_scientific_string()).as_deref(),
            "east" => self.east.map(|v| v.value.to_scientific_string()).as_deref(),
            "west" => self.west.map(|v| v.value.to_scientific_string()).as_deref(),
            "vendor" => self.vendor.as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(geo_reference) = &self.geo_reference {
            visit_children!(visitor, "geoReference" => geo_reference);
        }

        if let Some(offset) = &self.offset {
            visit_children!(visitor, "offset" => offset);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Header
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut geo_reference = None;
        let mut offset = None;
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "geoReference" => GeoReference => |v| geo_reference = Some(v),
            "offset" => Offset => |v| offset = Some(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            rev_major: read.attribute("revMajor")?,
            rev_minor: read.attribute("revMinor")?,
            name: read.attribute_opt("name")?,
            version: read.attribute_opt("version")?,
            date: read.attribute_opt("date")?,
            north: read.attribute_opt("north")?.map(Length::new::<meter>),
            south: read.attribute_opt("south")?.map(Length::new::<meter>),
            east: read.attribute_opt("east")?.map(Length::new::<meter>),
            west: read.attribute_opt("west")?.map(Length::new::<meter>),
            vendor: read.attribute_opt("vendor")?,
            geo_reference,
            offset,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Header {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            rev_major: u.arbitrary()?,
            rev_minor: u.arbitrary()?,
            name: u.arbitrary()?,
            version: u.arbitrary()?,
            date: u.arbitrary()?,
            north: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
            south: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
            east: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
            west: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
            vendor: u.arbitrary()?,
            geo_reference: u.arbitrary()?,
            offset: u.arbitrary()?,
            additional_data: u.arbitrary()?,
        })
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            rev_major: Self::default_rev_major(),
            rev_minor: Self::default_rev_minor(),
            name: None,
            version: None,
            date: Some(Self::default_date_now().to_string()),
            north: None,
            south: None,
            east: None,
            west: None,
            vendor: None,
            geo_reference: None,
            offset: None,
            additional_data: AdditionalData::default(),
        }
    }
}

impl Header {
    pub fn default_rev_major() -> u16 {
        1
    }

    pub fn default_rev_minor() -> u16 {
        7
    }

    pub fn default_date_now() -> DateTime<Utc> {
        chrono::Local::now().into()
    }

    fn parse_date(date: &str) -> Option<DateTime<Utc>> {
        // this is the format used in all the ASAM examples ...
        if let Ok(date) = NaiveDateTime::parse_from_str(date, "%a %h %e %H:%M:%S %Y") {
            Some(DateTime::<Utc>::from_utc(date, Utc))
        } else {
            DateTime::from_str(date).ok()
        }
    }

    pub fn date_parsed(&self) -> Option<DateTime<Utc>> {
        self.date.as_deref().and_then(Self::parse_date)
    }
}
