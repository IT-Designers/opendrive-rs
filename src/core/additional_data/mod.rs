use serde_derive::{Deserialize, Serialize};

pub mod data_quality;

/// ASAM OpenDRIVE offers the possibility to include external data. The processing of this data
/// depends on the application.
/// Additional data may be placed at any position in ASAM OpenDRIVE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AdditionalData {
    #[serde(rename = "dataQuality")]
    DataQuality(data_quality::DataQuality),
    #[serde(rename = "include")]
    Include(Vec<Include>),
    #[serde(rename = "userData")]
    UserData(Vec<UserData>),
}

/// ASAM OpenDRIVE allows including external files into the ASAM OpenDRIVE file. The processing of
/// the files depends on the application.
/// Included data is represented by `<include>` elements. They may be stored at any position in ASAM
/// OpenDRIVE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Include {
    /// Location of the file that is to be included
    pub file: String,
}

/// Ancillary data should be described near the element it refers to. Ancillary data contains data
/// that are not yet described in ASAM OpenDRIVE, or data that is needed by an application for a
/// specific reason. Examples are different road textures.
/// In ASAM OpenDRIVE, ancillary data is represented by `<userData>` elements. They may be stored at
/// any element in ASAM OpenDRIVE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserData {
    /// Code for the user data. Free text, depending on application.
    pub code: String,
    /// User data. Free text, depending on application.
    pub value: Option<String>,
}
