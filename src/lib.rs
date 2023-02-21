//! This documentation contains comments from the XML-Schemata definitions of the
//! [ASAM OpenDRIVE](https://www.asam.net/standards/detail/opendrive/) standard and therefore might
//! introduce further license restrictions.
//!
//! These XML-Schemata definitions contain the following file header:
//!
//! ```text
//! ASAM OpenDRIVE V1.7.0
//!
//! © by ASAM e.V., 2021
//!
//! ASAM OpenDRIVE defines a file format for the precise analytical description of
//! road networks
//!
//!
//! Any use is limited to the scope described in the ASAM license terms.
//! This file is distributable in accordance with the ASAM license terms.
//! See www.asam.net/license.html for further details.
//! ```
//!
//! See also the additional [disclaimer](https://www.asam.net/index.php?eID=dumpFile&t=f&f=4422&token=e590561f3c39aa2260e5442e29e93f6693d1cccd)
//! of the `ASAM OpenDRIVE V1.7.0 User Guide`:
//!
//! ```text
//! This document is the copyrighted property of ASAM e.V. In alteration to the regular license
//! terms, ASAM allows unrestricted distribution of this standard. §2 (1) of ASAM’s regular license
//! terms is therefore substituted by the following clause: "The licensor grants everyone a basic,
//! non-exclusive and unlimited license to use the standard ASAM OpenDRIVE".
//! ```
//! where `license terms` refers to https://www.asam.net/license

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
