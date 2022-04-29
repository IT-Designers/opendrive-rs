use crate::junction::junction_group::JunctionGroup;
use crate::junction::Junction;
use crate::road::railroad::station::Station;
use crate::road::signals::controller::Controller;
use crate::road::Road;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::borrow::Cow;
use std::str::FromStr;
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length};
use uom::si::length::meter;
use xml::{EventReader, EventWriter};

pub mod additional_data;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct OpenDrive {
    pub header: Header,
    pub road: Vec<Road>,
    pub controller: Vec<Controller>,
    pub junction: Vec<Junction>,
    pub junction_group: Vec<JunctionGroup>,
    pub station: Vec<Station>,
}

impl OpenDrive {
    pub fn from_reader<T: std::io::Read>(
        reader: EventReader<T>,
    ) -> Result<Self, crate::parser::Error> {
        let mut events = reader.into_iter();
        let mut drive = None;

        let mut read = crate::parser::ReadContext {
            iterator: &mut events,
            path: crate::parser::Path {
                parent: None,
                name: "",
            },
            attributes: Vec::new(),
            children_done: false,
        };

        match_child_eq_ignore_ascii_case!(
            read,
            "OpenDRIVE" true => OpenDrive => |v| drive = Some(v),
        );

        Ok(drive.unwrap())
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

        for road in &self.road {
            visit_children!(visitor, "road" => road);
        }

        for controller in &self.controller {
            visit_children!(visitor, "controller" => controller);
        }

        for junction in &self.junction {
            visit_children!(visitor, "junction" => junction);
        }

        for junction_group in &self.junction_group {
            visit_children!(visitor, "junctionGroup" => junction_group);
        }

        for station in &self.station {
            visit_children!(visitor, "station" => station);
        }

        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for OpenDrive
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut header = None;
        let mut roads = Vec::new();
        let mut controller = Vec::new();
        let mut junction = Vec::new();
        let mut junction_group = Vec::new();
        let mut station = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "header" true => Header => |v| header = Some(v),
            "road" => Road => |v| roads.push(v),
            "controller" => Controller => |v| controller.push(v),
            "junction" => Junction => |v| junction.push(v),
            "junctionGroup" => JunctionGroup => |v| junction_group.push(v),
            "station" => Station => |v| station.push(v),
        );

        Ok(Self {
            header: header.unwrap(),
            road: roads,
            controller,
            junction,
            junction_group,
            station,
        })
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Header
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut geo_reference = None;
        let mut offset = None;

        match_child_eq_ignore_ascii_case!(
            read,
            "geoReference" => GeoReference => |v| geo_reference = Some(v),
            "offset" => Offset => |v| offset = Some(v),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for GeoReference
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(_read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {})
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Offset
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            hdg: read.attribute("hdg").map(Angle::new::<radian>)?,
            x: read.attribute("x").map(Length::new::<meter>)?,
            y: read.attribute("y").map(Length::new::<meter>)?,
            z: read.attribute("z").map(Length::new::<meter>)?,
        })
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
