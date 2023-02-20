use opendrive::core::OpenDrive;

const XML: &str = r#"
<?xml version="1.0" standalone="yes"?>
<OpenDRIVE>
    <header revMajor="1" revMinor="7" name="" version="1.00" date="Mon Oct 28 14:02:13 2019" north="0.0000000000000000e+00" south="0.0000000000000000e+00" east="0.0000000000000000e+00" west="0.0000000000000000e+00">
    </header>
</OpenDRIVE>
"#;

fn main() {
    let opendrive = OpenDrive::from_xml_str(XML).unwrap();

    dbg!(&opendrive.header);
    dbg!(&opendrive.additional_data);

    for _ in &opendrive.road {}
    for _ in &opendrive.junction {}
    for _ in &opendrive.junction_group {}
    for _ in &opendrive.controller {}

    println!("XML: {}", opendrive.to_xml_string().unwrap());
}
