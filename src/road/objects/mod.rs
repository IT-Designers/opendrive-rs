use crate::road::objects::bridge::Bridge;
use crate::road::objects::markings::Markings;
use crate::road::objects::material::Material;
use crate::road::objects::outline::{Outline, Outlines};
use crate::road::objects::parking::ParkingSpace;
use crate::road::objects::reference::ObjectReference;
use crate::road::objects::repeat::Repeat;
use crate::road::objects::surface::Surface;
use crate::road::objects::tunnel::Tunnel;
use crate::road::objects::validity::LaneValidity;
use std::borrow::Cow;
use uom::si::angle::radian;
use uom::si::f64::Angle;
use uom::si::f64::Length;
use uom::si::length::meter;

pub mod borders;
pub mod bridge;
pub mod markings;
pub mod material;
pub mod outline;
pub mod parking;
pub mod reference;
pub mod repeat;
pub mod road_mark_color;
pub mod side_type;
pub mod surface;
pub mod tunnel;
pub mod validity;

/// Container for all objects along a road
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Objects {
    pub object: Vec<Object>,
    pub object_reference: Vec<ObjectReference>,
    pub tunnel: Vec<Tunnel>,
    pub bridge: Vec<Bridge>,
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

        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Objects
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut object = Vec::new();
        let mut object_reference = Vec::new();
        let mut tunnel = Vec::new();
        let mut bridge = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "object" => Object => |v| object.push(v),
            "objectReference" => ObjectReference => |v| object_reference.push(v),
            "tunnel" => Tunnel => |v| tunnel.push(v),
            "bridge" => Bridge => |v| bridge.push(v),
        );

        Ok(Self {
            object,
            object_reference,
            tunnel,
            bridge,
        })
    }
}

/// Describes common 3D objects that have a reference to a given road. Objects are items that
/// influence a road by expanding, delimiting, and supplementing its course. The most common
/// examples are parking spaces, crosswalks, and traffic barriers.
/// There are two ways to describe the bounding box of objects.
///   - For an angular object: definition of the width, length and height.
///   - For a circular object: definition of the radius and height.
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    /// Indicates whether the object is dynamic or static, default value is “no” (static). Dynamic
    /// object cannot change its position.
    pub dynamic: Option<bool>,
    /// Heading angle of the object relative to road direction
    pub hdg: Option<Angle>,
    /// Height of the object's bounding box. @height is defined in the local coordinate system u/v
    /// along the z-axis
    pub height: Option<Length>,
    /// Unique ID within database
    pub id: String,
    /// Length of the object's bounding box, alternative to @radius.
    /// @length is defined in the local coordinate system u/v along the v-axis
    pub length: Option<Length>,
    /// Name of the object. May be chosen freely.
    pub name: Option<String>,
    /// - "+" = valid in positive s-direction
    /// - "-" = valid in negative s-direction
    /// - "none" = valid in both directions
    /// (does not affect the heading)
    pub orientation: Option<Orientation>,
    /// Alternative to @pitch and @roll. If true, the object is vertically perpendicular to the road
    /// surface at all points and @pitch and @roll are ignored. Default is false.
    pub perp_to_road: Option<bool>,
    /// Pitch angle relative to the x/y-plane
    pub pitch: Option<Angle>,
    /// radius of the circular object's bounding box, alternative to @length and @width. @radius is
    /// defined in the local coordinate system u/v
    pub radius: Option<Length>,
    /// Roll angle relative to the x/y-plane
    pub roll: Option<Angle>,
    /// s-coordinate of object's origin
    pub s: Length,
    /// Variant of a type
    pub subtype: Option<String>,
    /// t-coordinate of object's origin
    pub t: Length,
    /// Type of object. For a parking space, the `<parkingSpace>` element may be used additionally.
    pub r#type: Option<ObjectType>,
    /// Validity of object along s-axis (0.0 for point object)
    pub valid_length: Option<Length>,
    /// Width of the object's bounding box, alternative to @radius.
    /// @width is defined in the local coordinate system u/v along the u-axis
    pub width: Option<Length>,
    /// z-offset of object's origin relative to the elevation of the reference line
    pub z_offset: Length,
    pub repeat: Vec<Repeat>,
    pub outline: Option<Outline>,
    pub outlines: Option<Outlines>,
    pub material: Vec<Material>,
    pub validity: Vec<LaneValidity>,
    pub parking_space: Option<ParkingSpace>,
    pub markings: Option<Markings>,
    pub borders: Option<Borders>,
    pub surface: Option<Surface>,
}

impl Object {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "dynamic" => self.dynamic.map(|v| if v { "yes" } else {"no"}),
            "hdg" => self.hdg.map(|v| v.value.to_scientific_string()).as_deref(),
            "height" => self.height.map(|v| v.value.to_scientific_string()).as_deref(),
            "id" => Some(self.id.as_str()),
            "length" => self.length.map(|v| v.value.to_scientific_string()).as_deref(),
            "name" => self.name.as_deref(),
            "orientation" => self.orientation.as_ref().map(Orientation::as_str),
            "perpToRoad" => self.perp_to_road.map(|v| v.to_string()).as_deref(),
            "pitch" => self.pitch.map(|v| v.value.to_scientific_string()).as_deref(),
            "radius" => self.radius.map(|v| v.value.to_scientific_string()).as_deref(),
            "roll" => self.roll.map(|v| v.value.to_scientific_string()).as_deref(),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "subtype" => self.subtype.as_deref(),
            "t" => Some(self.t.value.to_scientific_string()).as_deref(),
            "type" => self.r#type.as_ref().map(ObjectType::as_str),
            "validLength" => self.valid_length.map(|v| v.value.to_scientific_string()).as_deref(),
            "width" => self.width.map(|v| v.value.to_scientific_string()).as_deref(),
            "zOffset" => Some(self.z_offset.value.to_scientific_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for repeat in &self.repeat {
            visit_children!(visitor, "repeat" => repeat);
        }

        if let Some(outline) = &self.outline {
            visit_children!(visitor, "outline" => outline);
        }

        if let Some(outlines) = &self.outlines {
            visit_children!(visitor, "outlines" => outlines);
        }

        for material in &self.material {
            visit_children!(visitor, "material" => material);
        }

        for validity in &self.validity {
            visit_children!(visitor, "validity" => validity);
        }

        if let Some(parking_space) = &self.parking_space {
            visit_children!(visitor, "parkingSpace" => parking_space);
        }

        if let Some(markings) = &self.markings {
            visit_children!(visitor, "markings" => markings);
        }

        if let Some(surface) = &self.surface {
            visit_children!(visitor, "surface" => surface);
        }

        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Object
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut repeat = Vec::new();
        let mut outline = None;
        let mut outlines = None;
        let mut material = Vec::new();
        let mut validity = Vec::new();
        let mut parking_space = None;
        let mut markings = None;
        let mut borders = None;
        let mut surface = None;

        match_child_eq_ignore_ascii_case!(
            read,
            "repeat" => Repeat => |v| repeat.push(v),
            "outline" => Outline => |v| outline = Some(v),
            "outlines" => Outlines => |v| outlines = Some(v),
            "material" => Material => |v| material.push(v),
            "validity" => LaneValidity => |v| validity.push(v),
            "parkingSpace" => ParkingSpace => |v| parking_space = Some(v),
            "markings" => Markings => |v| markings = Some(v),
            "borders" => Borders => |v| borders = Some(v),
            "surface" => Surface => |v| surface = Some(v),
        );

        Ok(Self {
            dynamic: read
                .attribute_opt::<String>("dynamic")?
                .map(|v| v.eq_ignore_ascii_case("yes")),
            hdg: read.attribute_opt("hdg")?.map(Angle::new::<radian>),
            height: read.attribute_opt("height")?.map(Length::new::<meter>),
            id: read.attribute("id")?,
            length: read
                .attribute_opt::<f64>("length")?
                .map(Length::new::<meter>),
            name: read.attribute_opt("name")?,
            orientation: read.attribute_opt("orientation")?,
            perp_to_road: read.attribute_opt("perpToRoad")?,
            pitch: read
                .attribute_opt::<f64>("pitch")?
                .map(Angle::new::<radian>),
            radius: read
                .attribute_opt::<f64>("radius")?
                .map(Length::new::<meter>),
            roll: read.attribute_opt::<f64>("roll")?.map(Angle::new::<radian>),
            s: read.attribute("s").map(Length::new::<meter>)?,
            subtype: read.attribute_opt("subtype")?,
            t: read.attribute("t").map(Length::new::<meter>)?,
            r#type: read.attribute_opt("type")?,
            valid_length: read.attribute_opt("validLength")?.map(Length::new::<meter>),
            width: read.attribute_opt("width")?.map(Length::new::<meter>),
            z_offset: read.attribute("zOffset").map(Length::new::<meter>)?,
            repeat,
            outline,
            outlines,
            material,
            validity,
            parking_space,
            markings,
            borders,
            surface,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Object {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            dynamic: u.arbitrary()?,
            hdg: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Angle::new::<radian>),
            height: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            id: u.arbitrary()?,
            length: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            name: u.arbitrary()?,
            orientation: u.arbitrary()?,
            perp_to_road: u.arbitrary()?,
            pitch: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Angle::new::<radian>),
            radius: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            roll: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Angle::new::<radian>),
            s: Length::new::<meter>(u.not_nan_f64()?),
            subtype: u.arbitrary()?,
            t: Length::new::<meter>(u.not_nan_f64()?),
            r#type: u.arbitrary()?,
            valid_length: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            width: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            z_offset: Length::new::<meter>(u.not_nan_f64()?),
            repeat: u.arbitrary()?,
            outline: u.arbitrary()?,
            outlines: u.arbitrary()?,
            material: u.arbitrary()?,
            validity: u.arbitrary()?,
            parking_space: u.arbitrary()?,
            markings: u.arbitrary()?,
            surface: u.arbitrary()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Orientation {
    Plus,
    Minus,
    None,
}

impl_from_str_as_str!(
    Orientation,
    "+" => Plus,
    "-" => Minus,
    "none" => None,
);

use crate::road::objects::borders::Borders;
pub use allow_deprecated::ObjectType;

#[allow(deprecated)]
mod allow_deprecated {
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
    pub enum ObjectType {
        /// i.e. unknown
        None,
        /// for anything that is not further categorized
        Obstacle,
        #[deprecated]
        Car,
        Pole,
        Tree,
        Vegetation,
        Barrier,
        Building,
        ParkingSpace,
        Patch,
        Railing,
        TrafficIsland,
        Crosswalk,
        StreetLamp,
        Gantry,
        SoundBarrier,
        #[deprecated]
        Van,
        #[deprecated]
        Bus,
        #[deprecated]
        Trailer,
        #[deprecated]
        Bike,
        #[deprecated]
        Motorbike,
        #[deprecated]
        Tram,
        #[deprecated]
        Train,
        #[deprecated]
        Pedestrian,
        #[deprecated]
        Wind,
        RoadMark,
    }

    impl_from_str_as_str!(
        ObjectType,
        "none" => None,
        "obstacle" => Obstacle,
        "car" => Car,
        "pole" => Pole,
        "tree" => Tree,
        "vegetation" => Vegetation,
        "barrier" => Barrier,
        "building" => Building,
        "parkingSpace" => ParkingSpace,
        "patch" => Patch,
        "railing" => Railing,
        "trafficIsland" => TrafficIsland,
        "crosswalk" => Crosswalk,
        "streetLamp" => StreetLamp,
        "gantry" => Gantry,
        "soundBarrier" => SoundBarrier,
        "van" => Van,
        "bus" => Bus,
        "trailer" => Trailer,
        "bike" => Bike,
        "motorbike" => Motorbike,
        "tram" => Tram,
        "train" => Train,
        "pedestrian" => Pedestrian,
        "wind" => Wind,
        "roadMark" => RoadMark,
    );
}
