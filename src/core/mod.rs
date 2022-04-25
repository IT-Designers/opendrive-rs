use crate::road::Road;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::borrow::Cow;
use std::str::FromStr;
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length};
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;
use xml::{EventReader, EventWriter};

pub mod additional_data;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct OpenDrive {
    pub header: Header,
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
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut header = None;
        let mut roads = Vec::new();

        find_map_parse_elem!(
            events,
            "header" => |attributes| {
                header = Some(Header::from_events(events, attributes)?);
                Ok(())
            },
            "road" => |attributes| {
                roads.push(Road::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self {
            header: header.unwrap(),
            roads,
        })
    }

    pub fn to_writer(&self) -> xml::writer::Result<EventWriter<Vec<u8>>> {
        let mut writer = EventWriter::new(Vec::new());
        self.append_to_writer(&mut writer)?;
        Ok(writer)
    }

    pub fn append_to_writer<'b, T: std::io::Write + 'b>(
        &self,
        writer: &'b mut EventWriter<T>,
    ) -> xml::writer::Result<()> {
        writer.write(xml::writer::XmlEvent::StartDocument {
            version: xml::common::XmlVersion::Version10,
            encoding: None,
            standalone: Some(true),
        })?;
        self.visit_attributes(|attributes| {
            writer.write(xml::writer::XmlEvent::StartElement {
                name: xml::name::Name::local("OpenDRIVE"),
                attributes,
                namespace: std::borrow::Cow::Owned(xml::namespace::Namespace::empty()),
            })
        })?;
        self.visit_children(|event| writer.write(event))?;
        writer.write(xml::writer::XmlEvent::EndElement { name: None })?;
        Ok(())
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(visitor)
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(visitor, "header" => self.header);
        for road in &self.roads {
            visit_children!(visitor, "road" => road);
        }
        Ok(())
    }
}

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
        Ok(())
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
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
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

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(visitor)
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(visitor);
        Ok(())
    }
}

/// To avoid large coordinates, an offset of the whole dataset may be applied using the `<offset>`
/// element. It enables inertial relocation and re-orientation of datasets. The dataset is first
/// translated by @x, @y, and @z. Afterwards, it is rotated by @hdg around the new origin. Rotation
/// around the z-axis should be avoided. In ASAM OpenDRIVE, the offset of a database is represented
/// by the `<offset>` element within the `<header>` element.
#[derive(Debug, Clone, PartialEq, Default)]
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
            hdg: find_map_parse_attr!(attributes, "hdg", f64).map(Angle::new::<radian>)?,
            x: find_map_parse_attr!(attributes, "x", f64).map(Length::new::<meter>)?,
            y: find_map_parse_attr!(attributes, "y", f64).map(Length::new::<meter>)?,
            z: find_map_parse_attr!(attributes, "z", f64).map(Length::new::<meter>)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "hdg" => &self.hdg.value.to_scientific_string(),
            "x" => &self.x.value.to_scientific_string(),
            "y" => &self.y.value.to_scientific_string(),
            "z" => &self.z.value.to_scientific_string(),
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
impl arbitrary::Arbitrary<'_> for Offset {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            hdg: Angle::new::<radian>(u.not_nan_f64()?),
            x: Length::new::<meter>(u.not_nan_f64()?),
            y: Length::new::<meter>(u.not_nan_f64()?),
            z: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}
