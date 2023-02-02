use crate::core::post_processing::PostProcessing;
use crate::core::source::Source;
use std::borrow::Cow;

/// Some basic metadata containing information about raw data included in ASAM OpenDRIVE is
/// described by the `<rawData>` element within the `<dataQuality`> element.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
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

impl RawData {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "date" => Some(self.date.as_str()),
            "postProcessing" => Some(self.post_processing.as_str()),
            "postProcessingComment" => self.post_processing_comment.as_deref(),
            "source" => Some(self.source.as_str()),
            "sourceComment" => self.source_comment.as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(visitor);
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for RawData
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            date: read.attribute("date")?,
            post_processing: read.attribute("postProcessing")?,
            post_processing_comment: read.attribute_opt("postProcessingComment")?,
            source: read.attribute("source")?,
            source_comment: read.attribute_opt("sourceComment")?,
        })
    }
}
