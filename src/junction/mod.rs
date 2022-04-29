use crate::junction::connection::Connection;
use crate::junction::controller::Controller;
use crate::junction::priority::Priority;
use crate::junction::surface::Surface;
use crate::road::objects::Orientation;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use vec1::Vec1;

pub mod connection;
pub mod controller;
pub mod junction_group;
pub mod junction_reference;
pub mod priority;
pub mod surface;

#[derive(Debug, Clone, PartialEq)]
pub struct Junction {
    pub connection: Vec1<Connection>,
    pub priority: Vec<Priority>,
    pub controller: Vec<Controller>,
    pub surface: Option<Surface>,
    /// Unique ID within database
    pub id: String,
    /// The main road from which the connecting roads of the virtual junction branch off. This
    /// attribute is mandatory for virtual junctions and shall not be specified for other junction
    /// types.
    pub main_road: Option<String>,
    /// Name of the junction. May be chosen freely.
    pub name: Option<String>,
    /// Defines the relevance of the virtual junction according to the driving direction. This
    /// attribute is mandatory for virtual junctions and shall not be specified for other junction
    /// types. The enumerator "none" specifies that the virtual junction is valid in both
    /// directions.
    pub orientation: Option<Orientation>,
    /// End position of the virtual junction in the reference line coordinate system. This attribute
    /// is mandatory for virtual junctions and shall not be specified for other junction types.
    pub s_end: Option<Length>,
    /// Start position of the virtual junction in the reference line coordinate system. This
    /// attribute is mandatory for virtual junctions and shall not be specified for other junction
    /// types.
    pub s_start: Option<Length>,
    /// Type of the junction. Common junctions are of type "default". This attribute is mandatory
    /// for virtual junctions and direct junctions. If the attribute is not specified, the junction
    /// type is "default".
    pub r#type: Option<JunctionType>,
}

impl Junction {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "id" => Some(self.id.as_str()),
            "mainRoad" => self.main_road.as_deref(),
            "name" => self.name.as_deref(),
            "orientation" => self.orientation.as_ref().map(Orientation::as_str),
            "sEnd" => self.s_end.map(|v| v.value.to_scientific_string()).as_deref(),
            "sStart" => self.s_start.map(|v| v.value.to_scientific_string()).as_deref(),
            "type" => self.r#type.as_ref().map(JunctionType::as_str),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for connection in &self.connection {
            visit_children!(visitor, "connection" => connection);
        }

        for priority in &self.priority {
            visit_children!(visitor, "priority" => priority);
        }

        for controller in &self.controller {
            visit_children!(visitor, "controller" => controller);
        }

        if let Some(surface) = &self.surface {
            visit_children!(visitor, "surface" => surface);
        }

        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Junction
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut connection = Vec::new();
        let mut priority = Vec::new();
        let mut controller = Vec::new();
        let mut surface = None;

        match_child_eq_ignore_ascii_case!(
            read,
            "connection" true => Connection => |v| connection.push(v),
            "priority" => Priority => |v| priority.push(v),
            "controller" => Controller => |v| controller.push(v),
            "surface" => Surface => |v| surface = Some(v),
        );

        Ok(Self {
            connection: Vec1::try_from(connection).unwrap(),
            priority,
            controller,
            surface,
            id: read.attribute("id")?,
            main_road: read.attribute_opt("mainRoad")?,
            name: read.attribute_opt("name")?,
            orientation: read.attribute_opt("orientation")?,
            s_end: read.attribute_opt("sEnd")?.map(Length::new::<meter>),
            s_start: read.attribute_opt("sStart")?.map(Length::new::<meter>),
            r#type: read.attribute_opt("type")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Junction {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            connection: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            priority: u.arbitrary()?,
            controller: u.arbitrary()?,
            surface: u.arbitrary()?,
            id: u.arbitrary()?,
            main_road: u.arbitrary()?,
            name: u.arbitrary()?,
            orientation: u.arbitrary()?,
            s_start: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            s_end: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            r#type: u.arbitrary()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum JunctionType {
    Default,
    Virtual,
    Direct,
}

impl_from_str_as_str!(
    JunctionType,
    "default" => Default,
    "virtual" => Virtual,
    "direct" => Direct,
);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum ContactPoint {
    Start,
    End,
}

impl_from_str_as_str!(
    ContactPoint,
    "start" => Start,
    "end" => End,
);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum ElementDir {
    Plus,
    Minus,
}

impl_from_str_as_str!(
    ElementDir,
    "+" => Plus,
    "-" => Minus,
);
