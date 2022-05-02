use crate::core::data_quality::DataQuality;
use crate::core::include::Include;
use crate::core::user_data::UserData;

/// ASAM OpenDRIVE offers the possibility to include external data. The processing of this data
/// depends on the application.
/// Additional data may be placed at any position in ASAM OpenDRIVE.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct AdditionalData {
    pub data_quality: Option<DataQuality>,
    pub include: Vec<Include>,
    pub user_data: Vec<UserData>,
}

impl AdditionalData {
    pub fn append_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(data_quality) = &self.data_quality {
            visit_children!(visitor, "dataQuality" => data_quality);
        }

        for include in &self.include {
            visit_children!(visitor, "include" => include);
        }

        for user_data in &self.user_data {
            visit_children!(visitor, "userData" => user_data);
        }

        Ok(())
    }

    pub fn fill<I>(
        &mut self,
        read: crate::parser::ReadContext<I>,
    ) -> Result<(), crate::parser::Error>
    where
        I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
    {
        match read.element_name() {
            name if name.eq_ignore_ascii_case("dataQuality") => {
                self.data_quality = Some(DataQuality::try_from(read)?)
            }
            name if name.eq_ignore_ascii_case("include") => {
                self.include.push(Include::try_from(read)?)
            }
            name if name.eq_ignore_ascii_case("userData") => {
                self.user_data.push(UserData::try_from(read)?)
            }
            name => {
                return Err(crate::parser::Error::InvalidValueFor {
                    name: core::any::type_name::<Self>().to_string(),
                    value: name.to_string(),
                });
            }
        };
        Ok(())
    }
}
