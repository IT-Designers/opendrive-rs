#[macro_use]
pub mod parser;
pub mod writer;

pub mod core;
pub mod junction;
pub mod lane;
pub mod object;
pub mod railroad;
pub mod road;
pub mod signal;

#[cfg(feature = "fuzzing")]
pub mod fuzzing;

#[cfg(test)]
mod tests {
    use crate::core::OpenDrive;

    #[test]
    pub fn test_xml() {
        let source = r#"
                <?xml version="1.0" standalone="yes"?>
                <OpenDRIVE> 
                    <header revMajor="1" revMinor="7" name="" version="1.00" date="Tue Feb 25 13:02:27 2020" north="0.0000000000000000e+00" south="0.0000000000000000e+00" east="0.0000000000000000e+00" west="0.0000000000000000e+00">
                    </header>
                </OpenDRIVE>
            "#;
        let _ = OpenDrive::from_xml_str(source).unwrap();
    }

    #[test]
    pub fn test_xml_lane_choice() {
        let source = r#"
                <?xml version="1.0" standalone="yes"?>
                <OpenDRIVE> 
                    <header revMajor="1" revMinor="7" name="" version="1.00" date="Tue Feb 25 13:02:27 2020" north="0.0000000000000000e+00" south="0.0000000000000000e+00" east="0.0000000000000000e+00" west="0.0000000000000000e+00">
                    </header>
                    <road rule="RHT" name="" length="1.0000000000000000e+02" id="1" junction="-1">
                        <link>
                        </link>
                        <planView>
                            <geometry s="0.0000000000000000e+00" x="0.0000000000000000e+00" y="0.0000000000000000e+00" hdg="0.0000000000000000e+00" length="1.0000000000000000e+02">
                                <line/>
                            </geometry>
                        </planView>
                        <lateralProfile>
                        </lateralProfile>
                        <lanes>
                            <laneSection s="0.0000000000000000e+00">
                                <center>
                                    <lane id="1337" type="driving" level="false">
                                        <border sOffset="0.0000000000000000e+00" a="3.5699999999999998e+00" b="0.0000000000000000e+00" c="0.0000000000000000e+00" d="0.0000000000000000e+00"/>
                                    </lane>
                                </center>
                            </laneSection>
                        </lanes>
                    </road>
                </OpenDRIVE>
            "#;
        let _ = OpenDrive::from_xml_str(source).unwrap();
    }

    #[test]
    pub fn test_xml_events_center_lane() {
        let source = r#"
                <?xml version="1.0" standalone="yes"?>
                <OpenDRIVE> 
                    <header revMajor="1" revMinor="7" name="" version="1.00" date="Tue Feb 25 13:02:27 2020" north="0.0000000000000000e+00" south="0.0000000000000000e+00" east="0.0000000000000000e+00" west="0.0000000000000000e+00">
                    </header>
                    <road rule="RHT" name="" length="1.0000000000000000e+02" id="1" junction="-1">
                        <link>
                        </link>
                        <planView>
                            <geometry s="0.0000000000000000e+00" x="0.0000000000000000e+00" y="0.0000000000000000e+00" hdg="0.0000000000000000e+00" length="1.0000000000000000e+02">
                                <line/>
                            </geometry>
                        </planView>
                        <lateralProfile>
                        </lateralProfile>
                        <lanes>
                            <laneSection s="0.0000000000000000e+00">
                                <center>
                                    <lane id="1337" type="driving" level="false">
                                        <border sOffset="0.0000000000000000e+00" a="3.5699999999999998e+00" b="0.0000000000000000e+00" c="0.0000000000000000e+00" d="0.0000000000000000e+00"/>
                                    </lane>
                                </center>
                            </laneSection>
                        </lanes>
                    </road>
                </OpenDRIVE>
            "#;
        let _ = OpenDrive::from_xml_str(source).unwrap();
    }
}
