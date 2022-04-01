use uom::si::f64::Length;

/// Raw data or data from external sources that is integrated in ASAM OpenDRIVE may be of varying
/// quality. It is possible to describe quality and accuracy of external data in ASAM OpenDRIVE.
/// The description of the data quality is represented by `<dataQuality>` elements. They may be
/// stored at any position in ASAM OpenDRIVE.
/// Measurement data derived from external sources like GPS that is integrated in ASAM OpenDRIVE may
/// be inaccurate. The error range, given in `m`, may be listed in the application.
#[derive(Debug, Clone)]
pub struct DataQuality {
    pub error: Option<Error>,
    pub raw_data: Option<RawData>,
}

/// The absolute or relative errors of road data are described by `<error>` elements within the
/// `<dataQuality>` element.
#[derive(Debug, Clone)]
pub struct Error {
    /// Absolute error of the road data in x/y direct
    pub xy_absolute: Length,
    /// Relative error of the road data in x/y direction
    pub xy_relative: Length,
    /// Absolute error of the road data in z direction
    pub z_absolute: Length,
    /// Relative error of the road data in z direction
    pub z_relative: Length,
}

/// Some basic metadata containing information about raw data included in ASAM OpenDRIVE is
/// described by the `<rawData>` element within the `<dataQuality`> element.
#[derive(Debug, Clone)]
pub struct RawData {
    /// Date of the delivery of raw data, to be given in ISO 8601 notification
    /// (YYYY-MM-DDTHH:MM:SS) `[9]`.
    /// Time-of-day may be omitted.
    pub date: String,
    /// Information about the kind of data handling before exporting data into the ASAM OpenDRIVE
    /// file
    pub post_processing: PostProcessing,
    /// Comments concerning the postprocessing attribute. Free text, depending on the application
    pub post_processing_comment: Option<String>,
    /// Source that has been used for retrieving the raw data; further sources to be added in
    /// upcoming versions
    pub source: Source,
    /// Comments concerning the @source . Free text, depending on the application
    pub source_comment: Option<String>,
}

#[derive(Debug, Clone)]
pub enum PostProcessing {
    Raw,
    Cleaned,
    Processed,
    Fused,
}

#[derive(Debug, Clone)]
pub enum Source {
    Sensor,
    Cadaster,
    Custom,
}
