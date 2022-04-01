use crate::road::Road;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length};
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;
use xml::EventReader;

pub mod additional_data;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "OpenDRIVE")]
pub struct OpenDrive {
    #[serde(default = "OpenDrive::default_xmlns")]
    pub xmlns: String,
    pub header: Header,
    #[serde(rename = "road", default = "Vec::new")]
    pub roads: Vec<Road>,
}

impl OpenDrive {
    pub fn from_reader<T: std::io::Read>(
        reader: EventReader<T>,
    ) -> Result<Self, crate::parser::Error> {
        let mut events = reader.into_iter();
        let mut drive = None;

        find_map_parse_elem!(
            events,
            "OpenDRIVE" true => |attributes| {
                drive = Some(Self::from_events(&mut events, attributes)?);
                Ok(())
            }
        );

        Ok(drive.expect("Required element"))
    }

    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut header = None;
        let mut roads = Vec::new();

        find_map_parse_elem!(
            events,
            "header" true => |attributes| {
                header = Some(Header::from_events(events, attributes)?);
                Ok(())
            },
            "road" => |attributes| {
                roads.push(Road::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self {
            xmlns: find_map_parse_attr!(attributes, "xmlns", Option<String>)?
                .unwrap_or_else(Self::default_xmlns),
            header: header.unwrap(),
            roads,
        })
    }
}

impl Default for OpenDrive {
    fn default() -> Self {
        Self {
            xmlns: Self::default_xmlns(),
            header: Header::default(),
            roads: Vec::default(),
        }
    }
}

impl OpenDrive {
    pub fn default_xmlns() -> String {
        "https://code.asam.net/simulation/standard/opendrive_schema/".to_string()
    }
}

/// The `<header>` element is the very first element within the `<OpenDRIVE>` element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Header {
    #[serde(rename = "revMajor")]
    pub rev_major: u16,
    #[serde(rename = "revMinor")]
    rev_minor: u16,
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
}

impl Header {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut geo_reference = None;
        let mut offset = None;

        find_map_parse_elem!(
            events,
            "geoReference" => |attributes| {
                geo_reference = Some(GeoReference::from_events(events, attributes)?);
                Ok(())
            },
            "offset" => |attributes| {
                offset = Some(Offset::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self {
            rev_major: find_map_parse_attr!(attributes, "revMajor", u16)?,
            rev_minor: find_map_parse_attr!(attributes, "revMinor", u16)?,
            name: find_map_parse_attr!(attributes, "name", Option<String>)?,
            version: find_map_parse_attr!(attributes, "version", Option<String>)?,
            date: find_map_parse_attr!(attributes, "date", Option<String>)?,
            north: find_map_parse_attr!(attributes, "north", Option<f64>)?
                .map(Length::new::<meter>),
            south: find_map_parse_attr!(attributes, "south", Option<f64>)?
                .map(Length::new::<meter>),
            east: find_map_parse_attr!(attributes, "east", Option<f64>)?.map(Length::new::<meter>),
            west: find_map_parse_attr!(attributes, "west", Option<f64>)?.map(Length::new::<meter>),
            vendor: find_map_parse_attr!(attributes, "vendor", Option<String>)?,
            geo_reference,
            offset,
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

/// Spatial reference systems are standardized by the European Petroleum Survey Group Geodesy (EPSG)
/// and are defined by parameters describing the geodetic datum. A geodetic datum is a coordinate
/// reference system for a collection of positions that are relative to an ellipsoid model of the
/// earth.
/// A geodetic datum is described by a projection string according to PROJ, that is, a format for
/// the exchange of data between two coordinate systems. This data shall be marked as CDATA, because
/// it may contain characters that interfere with the XML syntax of an element’s attribute.
/// In ASAM OpenDRIVE, the information about the geographic reference of an ASAM OpenDRIVE dataset
/// is represented by the `<geoReference>` element within the `<header>` element.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeoReference {
    // TODO pub additional_data: Vec<AdditionalData>,
}

impl GeoReference {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let _ = attributes;
        find_map_parse_elem!(events);
        Ok(Self {})
    }
}

/// To avoid large coordinates, an offset of the whole dataset may be applied using the `<offset>`
/// element. It enables inertial relocation and re-orientation of datasets. The dataset is first
/// translated by @x, @y, and @z. Afterwards, it is rotated by @hdg around the new origin. Rotation
/// around the z-axis should be avoided. In ASAM OpenDRIVE, the offset of a database is represented
/// by the `<offset>` element within the `<header>` element.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Offset {
    /// Heading offset (rotation around resulting z-axis)
    pub hdg: Angle,
    /// Inertial x offset
    pub x: Length,
    /// Inertial y offset
    pub y: Length,
    /// Inertial z offset
    pub z: Length,
}

impl Offset {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            hdg: find_map_parse_attr!(attributes, "hdg", f64).map(|v| Angle::new::<radian>(v))?,
            x: find_map_parse_attr!(attributes, "x", f64).map(Length::new::<meter>)?,
            y: find_map_parse_attr!(attributes, "y", f64).map(Length::new::<meter>)?,
            z: find_map_parse_attr!(attributes, "z", f64).map(Length::new::<meter>)?,
        })
    }
}
