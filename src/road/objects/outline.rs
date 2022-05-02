use crate::road::lane::LaneType;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use vec1::Vec1;

/// An outline defines a series of corner points, including the height of the object relative to the
/// road reference line. The inner area of the described outline may be filled with a filling type,
/// such as grass, concrete, asphalt, or pavement.
#[derive(Debug, Clone, PartialEq)]
pub struct Outlines {
    pub outline: Vec1<Outline>,
}

impl Outlines {
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
        for outline in &self.outline {
            visit_children!(visitor, "outline" => outline);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Outlines
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut outline = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "outline" true => Outline => |v| outline.push(v),
        );

        Ok(Self {
            outline: Vec1::try_from_vec(outline).unwrap(),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Outlines {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            outline: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
        })
    }
}

/// Defines a series of corner points, including the height of the object relative to the road
/// reference line. For areas, the points should be listed in counter-clockwise order.
/// An `<outline>` element shall be followed by one or more `<cornerRoad>` element or by one or more
/// `<cornerLocal>` element.
/// ASAM OpenDRIVE 1.4 outline definitions (without `<outlines>` parent element) shall still be
/// supported.
#[derive(Debug, Clone, PartialEq)]
pub struct Outline {
    /// If true, the outline describes an area, not a linear feature
    pub closed: Option<bool>,
    /// Type used to fill the area inside the outline
    pub fill_type: Option<OutlineFillType>,
    /// ID of the outline. Must be unique within one object.
    pub id: Option<u64>,
    /// Describes the lane type of the outline
    pub lane_type: Option<LaneType>,
    /// Defines if outline is an outer outline of the object
    pub outer: Option<bool>,
    pub choice: Vec1<Corner>,
}

impl Outline {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "closed" => self.closed.map(|v| v.to_string()).as_deref(),
            "fillType" => self.fill_type.as_ref().map(OutlineFillType::as_str),
            "id" => self.id.map(|v| v.to_string()).as_deref(),
            "laneType" => self.lane_type.as_ref().map(LaneType::as_str),
            "outer" => self.outer.map(|v| v.to_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for choice in &self.choice {
            match choice {
                Corner::Road(road) => visit_children!(visitor, "cornerRoad" => road),
                Corner::Local(local) => visit_children!(visitor, "cornerLocal" => local),
            }
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Outline
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut choice = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "cornerRoad" => CornerRoad => |v| choice.push(Corner::Road(v)),
            "cornerLocal" => CornerLocal => |v| choice.push(Corner::Local(v)),
        );

        Ok(Self {
            closed: read.attribute_opt("closed")?,
            fill_type: read.attribute_opt("fillType")?,
            id: read.attribute_opt("id")?,
            lane_type: read.attribute_opt("laneType")?,
            outer: read.attribute_opt("outer")?,
            choice: Vec1::try_from_vec(choice).map_err(|_| {
                crate::parser::Error::missing_element(
                    read.path().to_string(),
                    "cornerRoad|cornerLocal",
                    core::any::type_name::<Corner>(),
                )
            })?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Outline {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            closed: u.arbitrary()?,
            fill_type: u.arbitrary()?,
            id: u.arbitrary()?,
            lane_type: u.arbitrary()?,
            outer: u.arbitrary()?,
            choice: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Corner {
    Road(CornerRoad),
    Local(CornerLocal),
}

/// Defines a corner point on the object’s outline in road coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct CornerRoad {
    /// dz of the corner relative to road reference line
    pub dz: Length,
    /// Height of the object at this corner, along the z-axis
    pub height: Length,
    /// ID of the outline point. Must be unique within one outline
    pub id: Option<u64>,
    /// s-coordinate of the corner
    pub s: Length,
    /// t-coordinate of the corner
    pub t: Length,
}

impl CornerRoad {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "dz" => Some(self.dz.value.to_scientific_string()).as_deref(),
            "height" => Some(self.height.value.to_scientific_string()).as_deref(),
            "id" => self.id.map(|v| v.to_string()).as_deref(),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "t" => Some(self.t.value.to_scientific_string()).as_deref(),
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
impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for CornerRoad
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            dz: read.attribute("dz").map(Length::new::<meter>)?,
            height: read.attribute("height").map(Length::new::<meter>)?,
            id: read.attribute_opt("id")?,
            s: read.attribute("s").map(Length::new::<meter>)?,
            t: read.attribute("t").map(Length::new::<meter>)?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for CornerRoad {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            dz: Length::new::<meter>(u.not_nan_f64()?),
            height: Length::new::<meter>(u.not_nan_f64()?),
            id: u.arbitrary()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
            t: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}

/// Used to describe complex forms of objects. Defines a corner point on the object outline relative
/// to the object pivot point in local u/v-coordinates. The insertion point and the orientation of
/// the object are given by the @s, @t, @zOffset and @hdg attributes of the  element.
#[derive(Debug, Clone, PartialEq)]
pub struct CornerLocal {
    /// Height of the object at this corner, along the z-axis
    pub height: Length,
    /// ID of the outline point. Shall be unique within one outline.
    pub id: Option<u64>,
    /// Local u-coordinate of the corner
    pub u: Length,
    /// Local v-coordinate of the corner
    pub v: Length,
    /// Local z-coordinate of the corner
    pub z: Length,
}

impl CornerLocal {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "height" => Some(self.height.value.to_scientific_string()).as_deref(),
            "id" => self.id.map(|v| v.to_string()).as_deref(),
            "u" => Some(self.u.value.to_scientific_string()).as_deref(),
            "v" => Some(self.v.value.to_scientific_string()).as_deref(),
            "z" => Some(self.z.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for CornerLocal
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            height: read.attribute("height").map(Length::new::<meter>)?,
            id: read.attribute_opt("id")?,
            u: read.attribute("u").map(Length::new::<meter>)?,
            v: read.attribute("v").map(Length::new::<meter>)?,
            z: read.attribute("z").map(Length::new::<meter>)?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for CornerLocal {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            height: Length::new::<meter>(u.not_nan_f64()?),
            id: u.arbitrary()?,
            u: Length::new::<meter>(u.not_nan_f64()?),
            v: Length::new::<meter>(u.not_nan_f64()?),
            z: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum OutlineFillType {
    Grass,
    Concrete,
    Cobble,
    Asphalt,
    Pavement,
    Gravel,
    Soil,
}

impl_from_str_as_str!(
    OutlineFillType,
    "grass" => Grass,
    "concrete" => Concrete,
    "cobble" => Cobble,
    "asphalt" => Asphalt,
    "pavement" => Pavement,
    "gravel" => Gravel,
    "soil" => Soil,
);
