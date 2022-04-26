#![no_main]

use libfuzzer_sys::fuzz_target;
use opendrive::core::OpenDrive;
use opendrive::xml::EventReader;

fuzz_target!(|data: OpenDrive| {
    let writer = data.to_writer().unwrap();
    let bytes = writer.into_inner();

    let reader = EventReader::from_str(core::str::from_utf8(&bytes).unwrap());
    let data_2 = OpenDrive::from_reader(reader);

    if data_2.is_err() {
        eprintln!("{}", data_2.as_ref().unwrap_err());
        dbg!(core::str::from_utf8(&bytes).unwrap());
    }

    let data_2 = data_2.unwrap();

    if data != data_2 {
        let a = format!("{data:?}");
        let b = format!("{data_2:?}");

        let overlap = a
            .chars()
            .zip(b.chars())
            .take_while(|(a, b)| a == b)
            .count()
            .saturating_sub(50);

        let a = &a[overlap..];
        let b = &b[overlap..];

        eprintln!("----- {overlap} || {} vs {}", a.len(), b.len());
        eprintln!("{}", &a[..50.min(a.len())]);
        eprintln!("{}", &b[..50.min(b.len())]);
        eprintln!("-----");

        if let Some(index) = b.find(": NaN") {
            let b = &b[index.saturating_sub(100)..];

            eprintln!("  ~~~~ NaN detected ~~~~ ");
            eprintln!("{}", &b[..100.min(b.len())]);
            eprintln!("  ~~~~ NaN detected ~~~~ ");
        }

        dbg!(core::str::from_utf8(&bytes).unwrap());
    }

    assert_eq!(data, data_2);
});
