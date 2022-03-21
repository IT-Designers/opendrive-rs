use crate::road::Road;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use uom::si::f64::{Angle, Length};
use url::Url;

pub mod additional_data;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "OpenDRIVE")]
pub struct OpenDrive {
    #[serde(default = "OpenDrive::default_xmlns")]
    pub xmlns: Url,
    pub header: Header,
    #[serde(rename = "road", default = "Vec::new")]
    pub roads: Vec<Road>,
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
    pub fn default_xmlns() -> Url {
        Url::from_str("https://code.asam.net/simulation/standard/opendrive_schema/")
            .expect("Valid default ASAM xmlns-url")
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
    #[serde(deserialize_with = "Header::deserialize_date")]
    pub date: Option<DateTime<Utc>>,
    pub north: Option<Length>,
    pub south: Option<Length>,
    pub east: Option<Length>,
    pub west: Option<Length>,
    pub vendor: Option<String>,
    pub geo_reference: Option<GeoReference>,
    pub offset: Option<Offset>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            rev_major: Self::default_rev_major(),
            rev_minor: Self::default_rev_minor(),
            name: None,
            version: None,
            date: Some(Self::default_date_now()),
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

    fn deserialize_date<'de, D>(content: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = <Option<String> as serde::Deserialize>::deserialize(content)?;
        content
            .map(|content| {
                // this is the format used in all the ASAM examples ...
                if let Ok(date) = NaiveDateTime::parse_from_str(&content, "%a %h %e %H:%M:%S %Y") {
                    Ok(DateTime::<Utc>::from_utc(date, Utc))
                } else {
                    DateTime::from_str(&content).map_err(serde::de::Error::custom)
                }
            })
            .transpose()
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
