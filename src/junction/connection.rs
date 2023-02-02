use crate::junction::connection_type::ConnectionType;
use crate::junction::contact_point::ContactPoint;
use crate::junction::lane_link::LaneLink;
use crate::junction::predecessor_successor::PredecessorSuccessor;
use std::borrow::Cow;

/// Provides information about a single connection within a junction.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Connection {
    pub predecessor: Option<PredecessorSuccessor>,
    pub successor: Option<PredecessorSuccessor>,
    pub lane_link: Vec<LaneLink>,
    /// ID of the connecting road
    pub connecting_road: Option<String>,
    /// Contact point on the connecting road
    pub contact_point: Option<ContactPoint>,
    /// Unique ID within the junction
    pub id: String,
    /// ID of the incoming road
    pub incoming_road: Option<String>,
    /// ID of the directly linked road. Only to be used for junctions of @type="direct".
    pub linked_road: Option<String>,
    /// Type of the connection. Regular connections are @type=“default”. This attribute is
    /// mandatory for virtual connections.
    pub r#type: Option<ConnectionType>,
}

impl Connection {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "connectingRoad" => self.connecting_road.as_deref(),
            "contactPoint" => self.contact_point.as_ref().map(ContactPoint::as_str),
            "id" => Some(self.id.as_str()),
            "incomingRoad" => self.incoming_road.as_deref(),
            "linkedRoad" => self.linked_road.as_deref(),
            "type" => self.r#type.as_ref().map(ConnectionType::as_str),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(predecessor) = &self.predecessor {
            visit_children!(visitor, "predecessor" => predecessor);
        }

        if let Some(successor) = &self.successor {
            visit_children!(visitor, "successor" => successor);
        }

        for lane_link in &self.lane_link {
            visit_children!(visitor, "laneLink" => lane_link);
        }

        Ok(())
    }
}
impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Connection
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut predecessor = None;
        let mut successor = None;
        let mut lane_link = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "predecessor" => PredecessorSuccessor => |v| predecessor = Some(v),
            "successor" => PredecessorSuccessor => |v| successor = Some(v),
            "laneLink" => LaneLink => |v| lane_link.push(v),
        );

        Ok(Self {
            predecessor,
            successor,
            lane_link,
            connecting_road: read.attribute_opt("connectingRoad")?,
            contact_point: read.attribute_opt("contactPoint")?,
            id: read.attribute("id")?,
            incoming_road: read.attribute_opt("incomingRoad")?,
            linked_road: read.attribute_opt("linkedRoad")?,
            r#type: read.attribute_opt("type")?,
        })
    }
}
