use crate::junction::{ContactPoint, ElementDir};
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

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
    type Error = crate::parser::Error;

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

/// Provides detailed information about the predecessor / successor road of a virtual connection.
/// Currently, only the @elementType “road” is allowed.
#[derive(Debug, Clone, PartialEq)]
pub struct PredecessorSuccessor {
    /// Direction, relative to the s-direction, of the connection on the preceding / succeeding road
    pub element_dir: ElementDir,
    /// ID of the linked element
    pub element_id: String,
    /// s-coordinate where the connection meets the preceding / succeding road.
    pub element_s: Length,
    /// Type of the linked element. Currently only "road" is allowed.
    pub element_type: String,
}
impl PredecessorSuccessor {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "elementDir" => self.element_dir.as_str(),
            "elementId" => self.element_id.as_str(),
            "elementS" => self.element_s.value.to_scientific_string().as_str(),
            "elementType" => self.element_type.as_str(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for PredecessorSuccessor
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            element_dir: read.attribute("elementDir")?,
            element_id: read.attribute("elementId")?,
            element_s: read.attribute("elementS").map(Length::new::<meter>)?,
            element_type: read.attribute("elementType")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for PredecessorSuccessor {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            element_dir: u.arbitrary()?,
            element_id: u.arbitrary()?,
            element_s: Length::new::<meter>(u.not_nan_f64()?),
            element_type: u.arbitrary()?,
        })
    }
}

/// Provides information about the lanes that are linked between an incoming road and a connecting
/// road. It is strongly recommended to provide this element. It is deprecated to omit the
/// `<laneLink>` element.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct LaneLink {
    /// ID of the incoming lane
    pub from: i64,
    /// ID of the connection lane
    pub to: i64,
}

impl LaneLink {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "from" => &self.from.to_string(),
            "to" => &self.to.to_string(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for LaneLink
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            from: read.attribute("from")?,
            to: read.attribute("to")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum ConnectionType {
    Default,
    Virtual,
}

impl_from_str_as_str!(
    ConnectionType,
    "default" => Default,
    "virtual" => Virtual,
);
