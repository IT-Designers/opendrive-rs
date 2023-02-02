use crate::core::error::Error;
use crate::core::raw_data::RawData;
use std::borrow::Cow;

/// Raw data or data from external sources that is integrated in ASAM OpenDRIVE may be of varying
/// quality. It is possible to describe quality and accuracy of external data in ASAM OpenDRIVE.
/// The description of the data quality is represented by `<dataQuality>` elements. They may be
/// stored at any position in ASAM OpenDRIVE.
/// Measurement data derived from external sources like GPS that is integrated in ASAM OpenDRIVE may
/// be inaccurate. The error range, given in `m`, may be listed in the application.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct DataQuality {
    pub error: Option<Error>,
    pub raw_data: Option<RawData>,
}

impl DataQuality {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(visitor)
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(error) = &self.error {
            visit_children!(visitor, "error" => error);
        }

        if let Some(raw_data) = &self.raw_data {
            visit_children!(visitor, "rawData" => raw_data);
        }

        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for DataQuality
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut error = None;
        let mut raw_data = None;

        match_child_eq_ignore_ascii_case!(
            read,
            "error" => Error => |v| error = Some(v),
            "rawData" => RawData => |v| raw_data = Some(v),
        );

        Ok(Self { error, raw_data })
    }
}
