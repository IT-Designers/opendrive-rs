use crate::road::railroad::main_track::MainTrack;
use crate::road::railroad::partner::Partner;
use crate::road::railroad::side_track::SideTrack;
use crate::road::railroad::switch_position::SwitchPosition;
use std::borrow::Cow;

pub mod main_track;
pub mod partner;
pub mod platform;
pub mod segment;
pub mod side_track;
pub mod station;
pub mod switch_position;

/// Container for all railroad definitions that shall be applied along a road.
/// The available set of railroad elements is currently limited to the definition of switches. All
/// other entries shall be covered with the existing elements, for example, track definition by
/// `<road>`, signal definition by `<signal>`, etc. Railroad-specific elements are defined against
/// the background of streetcar applications.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Railroad {
    pub switch: Vec<Switch>,
}

impl Railroad {
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
        for switch in &self.switch {
            visit_children!(visitor, "switch" => switch);
        }
        Ok(())
    }
}
impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Railroad
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut switch = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "switch" => Switch => |v| switch.push(v),
        );

        Ok(Self { switch })
    }
}

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

        Ok(())
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

        match_child_eq_ignore_ascii_case!(
            read,
            "mainTrack" true => MainTrack => |v| main_track = Some(v),
            "sideTrack" true => SideTrack => |v| side_track = Some(v),
            "partner" => Partner => |v| partner = Some(v),
        );

        Ok(Self {
            main_track: main_track.unwrap(),
            side_track: side_track.unwrap(),
            partner,
            id: read.attribute("id")?,
            name: read.attribute("name")?,
            position: read.attribute("position")?,
        })
    }
}
