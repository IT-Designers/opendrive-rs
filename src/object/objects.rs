use crate::core::additional_data::AdditionalData;
use crate::object::bridge::Bridge;
use crate::object::reference::ObjectReference;
use crate::object::tunnel::Tunnel;
use crate::object::Object;
use std::borrow::Cow;

/// Container for all objects along a road
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Objects {
    pub object: Vec<Object>,
    pub object_reference: Vec<ObjectReference>,
    pub tunnel: Vec<Tunnel>,
    pub bridge: Vec<Bridge>,
    pub additional_data: AdditionalData,
}

impl Objects {
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
        for object in &self.object {
            visit_children!(visitor, "object" => object);
        }

        for object_reference in &self.object_reference {
            visit_children!(visitor, "objectReference" => object_reference);
        }

        for tunnel in &self.tunnel {
            visit_children!(visitor, "tunnel" => tunnel);
        }

        for bridge in &self.bridge {
            visit_children!(visitor, "bridge" => bridge);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Objects
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut object = Vec::new();
        let mut object_reference = Vec::new();
        let mut tunnel = Vec::new();
        let mut bridge = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "object" => Object => |v| object.push(v),
            "objectReference" => ObjectReference => |v| object_reference.push(v),
            "tunnel" => Tunnel => |v| tunnel.push(v),
            "bridge" => Bridge => |v| bridge.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            object,
            object_reference,
            tunnel,
            bridge,
            additional_data,
        })
    }
}
