#![no_main]

#[macro_use]
extern crate opendrive;

use libfuzzer_sys::fuzz_target;
use opendrive::road::Road;
use opendrive::xml;
use opendrive::xml::writer::XmlEvent;
use opendrive::xml::{EventReader, EventWriter};
use std::borrow::Cow;

fuzz_target!(|data: Road| {
    let mut bytes = Vec::new();
    let mut writer = EventWriter::new(&mut bytes);

    data.visit_attributes(|attributes| {
        writer.write(XmlEvent::StartElement {
            name: xml::name::Name::local("road"),
            attributes: attributes,
            namespace: Cow::Owned(xml::namespace::Namespace::empty()),
        })
    })
    .unwrap();

    data.visit_children(|event| writer.write(event)).unwrap();
    writer.write(XmlEvent::EndElement { name: None }).unwrap();

    let reader = EventReader::from_str(core::str::from_utf8(&bytes).unwrap());
    let events = &mut reader.into_iter();

    let _ = (|| {
        find_map_parse_elem!(
            events,
            "road" true => |attributes| {
                let data_2 = Road::from_events(events, attributes)?;
                if data != data_2 {
                    dbg!(core::str::from_utf8(&bytes).unwrap());
                }
                assert_eq!(data, data_2);
                Ok(())
            }
        );
        Ok(())
    })()
    .unwrap();
});