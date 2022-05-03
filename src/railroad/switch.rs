use crate::core::additional_data::AdditionalData;
use crate::railroad::main_track::MainTrack;
use crate::railroad::partner::Partner;
use crate::railroad::side_track::SideTrack;
use crate::railroad::switch_position::SwitchPosition;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Switch {
    pub main_track: MainTrack,
    pub side_track: SideTrack,
    pub partner: Option<Partner>,
    /// Unique ID of the switch; preferably an integer number, see uint32_t
    pub id: String,
    /// Unique name of the switch
    pub name: String,
    /// Either a switch can be operated (dynamic) or it is in a static position
    pub position: SwitchPosition,
    pub additional_data: AdditionalData,
}

impl Switch {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "id" => self.id.as_str(),
            "name" => self.name.as_str(),
            "position" => self.position.as_str(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(
            visitor,
            "mainTrack" => self.main_track,
            "sideTrack" => self.side_track,
        );

        if let Some(partner) = &self.partner {
            visit_children!(visitor, "partner" => partner);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Switch
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut main_track = None;
        let mut side_track = None;
        let mut partner = None;
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "mainTrack" true => MainTrack => |v| main_track = Some(v),
            "sideTrack" true => SideTrack => |v| side_track = Some(v),
            "partner" => Partner => |v| partner = Some(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            main_track: main_track.unwrap(),
            side_track: side_track.unwrap(),
            partner,
            id: read.attribute("id")?,
            name: read.attribute("name")?,
            position: read.attribute("position")?,
            additional_data,
        })
    }
}
