#![no_main]

use libfuzzer_sys::fuzz_target;
use opendrive::core::OpenDrive;

fuzz_target!(|data: OpenDrive| {
    let string = data.to_xml_string().unwrap();
    let data_2 = OpenDrive::from_xml_str(&string);

    if let Err(e) = &data_2 {
        eprintln!("{e}");
        dbg!(&string);
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
        eprintln!("{}", &a[..150.min(a.len())]);
        eprintln!("{}", &b[..150.min(b.len())]);
        eprintln!("-----");

        if let Some(index) = b.find("NaN") {
            let b = &b[index.saturating_sub(50)..];

            eprintln!("  ~~~~ NaN detected ~~~~ ");
            eprintln!("{}", &b[..150.min(b.len())]);
            eprintln!("  ~~~~ NaN detected ~~~~ ");
        }

        // dbg!(core::str::from_utf8(&bytes).unwrap());
    }

    assert_eq!(data, data_2);
});
