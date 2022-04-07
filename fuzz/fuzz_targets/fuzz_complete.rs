#![no_main]

use libfuzzer_sys::fuzz_target;
use opendrive::core::OpenDrive;
use opendrive::xml::EventReader;

fuzz_target!(|data: OpenDrive| {
    let writer = data.to_writer().unwrap();
    let bytes = writer.into_inner();

    let reader = EventReader::from_str(core::str::from_utf8(&bytes).unwrap());
    let data_2 = OpenDrive::from_reader(reader).unwrap();

    if data != data_2 {
        dbg!(core::str::from_utf8(&bytes).unwrap());
    }

    assert_eq!(data, data_2);
});
