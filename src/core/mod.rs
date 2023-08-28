use crate::core::additional_data::AdditionalData;
use crate::core::header::Header;
use crate::junction::junction_group::JunctionGroup;
use crate::junction::Junction;
use crate::railroad::station::Station;
use crate::road::Road;
use crate::signal::controller::Controller;
use std::borrow::Cow;
use xml::{EventReader, EventWriter};

pub mod additional_data;
pub mod data_quality;
pub mod error;
pub mod geo_reference;
pub mod header;
pub mod include;
pub mod offset;
pub mod post_processing;
pub mod raw_data;
pub mod source;
pub mod user_data;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct OpenDrive {
    pub header: Header,
    pub road: Vec<Road>,
    pub controller: Vec<Controller>,
    pub junction: Vec<Junction>,
    pub junction_group: Vec<JunctionGroup>,
    pub station: Vec<Station>,
    pub additional_data: AdditionalData,
}

impl OpenDrive {
    #[inline]
    pub fn from_xml_str(s: &str) -> crate::parser::Result<Self> {
        Self::from_reader(EventReader::from_str(s.trim()))
    }

    #[inline]
    pub fn from_xml_read<T: std::io::Read>(r: T) -> crate::parser::Result<Self> {
        Self::from_reader(EventReader::new(r))
    }

    pub fn from_reader<T: std::io::Read>(reader: EventReader<T>) -> crate::parser::Result<Self> {
        let mut events = reader.into_iter();
        let mut drive = None;

        let mut read = crate::parser::ReadContext::from(&mut events);

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

    #[inline]
    pub fn to_xml_string(&self) -> crate::writer::Result<String> {
        String::from_utf8(
            self.to_writer()
                .map_err(crate::writer::Error::from)
                .map_err(Box::new)?
                .into_inner(),
        )
        .map_err(crate::writer::Error::from)
        .map_err(Box::new)
    }

    #[inline]
    pub fn to_xml_write(&self, w: impl std::io::Write) -> crate::writer::Result<()> {
        let mut writer = EventWriter::new(w);
        self.append_to_writer(&mut writer)
            .map_err(crate::writer::Error::from)
            .map_err(Box::new)?;
        Ok(())
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

        self.additional_data.append_children(visitor)
    }
}
impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for OpenDrive
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut header = None;
        let mut roads = Vec::new();
        let mut controller = Vec::new();
        let mut junction = Vec::new();
        let mut junction_group = Vec::new();
        let mut station = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "header" true => Header => |v| header = Some(v),
            "road" => Road => |v| roads.push(v),
            "controller" => Controller => |v| controller.push(v),
            "junction" => Junction => |v| junction.push(v),
            "junctionGroup" => JunctionGroup => |v| junction_group.push(v),
            "station" => Station => |v| station.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            header: header.unwrap(),
            road: roads,
            controller,
            junction,
            junction_group,
            station,
            additional_data,
        })
    }
}
