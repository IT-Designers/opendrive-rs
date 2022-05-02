use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use vec1::Vec1;

/// Defines the style of the line at the outer border of a lane. The style of the center line that
/// separates left and right lanes is determined by the road mark element for the center lane.
#[derive(Debug, Clone, PartialEq)]
pub struct RoadMark {
    pub sway: Vec<Sway>,
    pub r#type: Option<Type>,
    pub explicit: Option<Explicit>,
    /// Color of the road mark
    pub color: Color,
    /// Height of road mark above the road, i.e. thickness of the road mark
    pub height: Option<Length>,
    /// Allows a lane change in the indicated direction, taking into account that lanes are numbered
    /// in ascending order from right to left. If the attribute is missing, “both” is used as
    /// default.
    pub lane_change: Option<LaneChange>,
    /// Material of the road mark. Identifiers to be defined by the user, use "standard" as default
    /// value.
    pub material: Option<String>,
    /// s-coordinate of start position of the `<roadMark>` element, relative to the position of the
    /// preceding `<laneSection>` element
    pub s_offset: Length,
    /// Type of the road mark
    pub type_simplified: TypeSimplified,
    /// Weight of the road mark. This attribute is optional if detailed definition is given below.
    pub weight: Option<Weight>,
    /// Width of the road mark. This attribute is optional if detailed definition is given by
    /// <line> element.
    pub width: Option<Length>,
}

impl RoadMark {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "color" => Some(self.color.as_str()),
            "height" => self.height.map(|v| v.value.to_scientific_string()).as_deref(),
            "laneChange" => self.lane_change.as_ref().map(LaneChange::as_str),
            "material" => self.material.as_deref(),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
            "type" => Some(self.type_simplified.as_str()),
            "weight" => self.weight.as_ref().map(Weight::as_str),
            "width" => self.width.map(|v| v.value.to_scientific_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for sway in &self.sway {
            visit_children!(visitor, "sway" => sway);
        }

        if let Some(r#type) = &self.r#type {
            visit_children!(visitor, "type" => r#type);
        }

        if let Some(explicit) = &self.explicit {
            visit_children!(visitor, "explicit" => explicit);
        }

        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for RoadMark
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut sway = Vec::new();
        let mut r#type = None;
        let mut explicit = None;

        match_child_eq_ignore_ascii_case!(
            read,
            "sway" => Sway => |v| sway.push(v),
            "type" => Type => |v| r#type = Some(v),
            "explicit" => Explicit => |v| explicit = Some(v),
        );

        Ok(Self {
            sway,
            r#type,
            explicit,
            color: read.attribute("color")?,
            height: read.attribute_opt("height")?.map(Length::new::<meter>),
            lane_change: read.attribute_opt("laneChange")?,
            material: read.attribute_opt("material")?,
            s_offset: read.attribute("sOffset").map(Length::new::<meter>)?,
            type_simplified: read.attribute("type")?,
            weight: read.attribute_opt("weight")?,
            width: read.attribute_opt("width")?.map(Length::new::<meter>),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for RoadMark {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            sway: u.arbitrary()?,
            r#type: u.arbitrary()?,
            explicit: u.arbitrary()?,
            color: u.arbitrary()?,
            height: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
            lane_change: u.arbitrary()?,
            material: u.arbitrary()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            type_simplified: u.arbitrary()?,
            weight: u.arbitrary()?,
            width: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
        })
    }
}

/// Relocates the lateral reference position for the following (explicit) type definition and thus
/// defines an offset. The sway offset is relative to the nominal reference position of the lane
/// marking, meaning the lane border.
#[derive(Debug, Clone, PartialEq)]
pub struct Sway {
    /// Polynom parameter a, sway value at @s (ds=0)
    a: f64,
    /// Polynom parameter b
    b: f64,
    /// Polynom parameter c
    c: f64,
    /// Polynom parameter d
    d: f64,
    /// s-coordinate of start position of the `<sway>` element, relative to the @sOffset given in
    /// the `<roadMark>` element
    d_s: f64,
}

impl Sway {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "a" => Some(self.a.to_scientific_string()).as_deref(),
            "b" => Some(self.b.to_scientific_string()).as_deref(),
            "c" => Some(self.c.to_scientific_string()).as_deref(),
            "d" => Some(self.d.to_scientific_string()).as_deref(),
            "d_s" => Some(self.d_s.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Sway
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            a: read.attribute("a")?,
            b: read.attribute("b")?,
            c: read.attribute("c")?,
            d: read.attribute("d")?,
            d_s: read.attribute("d_s")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Sway {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            a: u.not_nan_f64()?,
            b: u.not_nan_f64()?,
            c: u.not_nan_f64()?,
            d: u.not_nan_f64()?,
            d_s: u.not_nan_f64()?,
        })
    }
}

/// Each type definition shall contain one or more line definitions with additional information
/// about the lines that the road mark is composed of.
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub line: Vec1<TypeLine>,
    /// Name of the road mark type. May be chosen freely.
    pub name: String,
    /// Accumulated width of the road mark. In case of several `<line>` elements this @width is the
    /// sum of all @width of `<line>` elements and spaces in between, necessary to form the road
    /// mark. This attribute supersedes the definition in the `<roadMark>` element.
    pub width: Length,
}

impl Type {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "name" => &self.name,
            "width" => &self.width.value.to_scientific_string(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for line in &self.line {
            visit_children!(visitor, "line" => line);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Type
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut line = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "line" true => TypeLine => |v| line.push(v),
        );

        Ok(Self {
            line: Vec1::try_from_vec(line).unwrap(),
            name: read.attribute("name")?,
            width: read.attribute("width").map(Length::new::<meter>)?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Type {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            line: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            name: u.arbitrary()?,
            width: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}

/// A road mark may consist of one or more elements. Multiple elements are usually positioned
/// side-by-side. A line definition is valid for a given length of the lane and will be repeated
/// automatically.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeLine {
    /// Line color. If given, this attribute supersedes the definition in the `<roadMark>` element.
    pub color: Option<Color>,
    /// Length of the visible part
    pub length: Length,
    /// Rule that must be observed when passing the line from inside, for example, from the lane
    /// with the lower absolute ID to the lane with the higher absolute ID
    pub rule: Option<Rule>,
    /// Initial longitudinal offset of the line definition from the start of the road mark
    /// definition
    pub s_offset: Length,
    /// Length of the gap between the visible parts
    pub space: Length,
    /// Lateral offset from the lane border.
    /// If `<sway>` element is present, the lateral offset follows the sway.
    pub t_offset: Length,
    /// Line width
    pub width: Option<Length>,
}

impl TypeLine {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "color" => self.color.as_ref().map(Color::as_str),
            "length" => Some(self.length.value.to_scientific_string()).as_deref(),
            "rule" => self.rule.as_ref().map(Rule::as_str),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
            "space" => Some(self.space.value.to_scientific_string()).as_deref(),
            "tOffset" => Some(self.t_offset.value.to_scientific_string()).as_deref(),
            "width" => self.width.map(|v| v.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for TypeLine
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            color: read.attribute_opt("color")?,
            length: read.attribute("length").map(Length::new::<meter>)?,
            rule: read.attribute_opt("rule")?,
            s_offset: read.attribute("sOffset").map(Length::new::<meter>)?,
            space: read.attribute("space").map(Length::new::<meter>)?,
            t_offset: read.attribute("tOffset").map(Length::new::<meter>)?,
            width: read.attribute_opt("width")?.map(Length::new::<meter>),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for TypeLine {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            color: u.arbitrary()?,
            length: Length::new::<meter>(u.not_nan_f64()?),
            rule: u.arbitrary()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            space: Length::new::<meter>(u.not_nan_f64()?),
            t_offset: Length::new::<meter>(u.not_nan_f64()?),
            width: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
        })
    }
}

/// Irregular road markings that cannot be described by repetitive line patterns may be described by
/// individual road marking elements. These explicit definitions also contain `<line>` elements for
/// the line definition, however, these lines will not be repeated automatically as in repetitive
/// road marking types. In ASAM OpenDRIVE, irregular road marking types and lines are represented by
/// `<explicit>` elements within elements. The line definitions are contained in `<line>` elements
/// within the `<explicit>` element.
// The `<explicit>` element should specifically be used for measurement data.
#[derive(Debug, Clone, PartialEq)]
pub struct Explicit {
    pub line: Vec1<ExplicitLine>,
}

impl Explicit {
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
        for line in &self.line {
            visit_children!(visitor, "line" => line);
        }
        Ok(())
    }
}
impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Explicit
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut line = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "line" true => ExplicitLine => |v| line.push(v),
        );

        Ok(Self {
            line: Vec1::try_from_vec(line).unwrap(),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Explicit {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            line: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExplicitLine {
    /// Length of the visible line
    pub length: Length,
    /// Rule that must be observed when passing the line from inside, that is, from the lane with
    /// the lower absolute ID to the lane with the higher absolute ID
    pub rule: Option<Rule>,
    /// Offset of start position of the `<line>` element, relative to the @sOffset  given in the
    /// `<roadMark>` element
    pub s_offset: Length,
    /// Lateral offset from the lane border. If `<sway>` element is present, the lateral offset
    /// follows the sway.
    pub t_offset: Length,
    /// Line width. This attribute supersedes the definition in the `<roadMark>` element.
    pub width: Option<Length>,
}

impl ExplicitLine {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "length" => Some(self.length.value.to_scientific_string()).as_deref(),
            "rule" => self.rule.as_ref().map(Rule::as_str),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
            "tOffset" => Some(self.t_offset.value.to_scientific_string()).as_deref(),
            "width" => self.width.map(|v| v.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for ExplicitLine
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            length: read.attribute("length").map(Length::new::<meter>)?,
            rule: read.attribute_opt("rule")?,
            s_offset: read.attribute("sOffset").map(Length::new::<meter>)?,
            t_offset: read.attribute("tOffset").map(Length::new::<meter>)?,
            width: read.attribute_opt("width")?.map(Length::new::<meter>),
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for ExplicitLine {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            length: Length::new::<meter>(u.not_nan_f64()?),
            rule: u.arbitrary()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            t_offset: Length::new::<meter>(u.not_nan_f64()?),
            width: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Rule {
    NoPassing,
    Caution,
    None,
}

impl_from_str_as_str!(
    Rule,
    "no passing" => NoPassing,
    "caution" => Caution,
    "none" => None,
);

/// The known keywords for the road mark color information
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Color {
    /// equivalent to [`Color::White`]
    Standard,
    Blue,
    Green,
    Red,
    White,
    Yellow,
    Orange,
    Violet,
}

impl_from_str_as_str!(
    Color,
    "standard" => Standard,
    "blue" => Blue,
    "green" => Green,
    "red" => Red,
    "white" => White,
    "yellow" => Yellow,
    "orange" => Orange,
    "violet" => Violet,
);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum LaneChange {
    Increase,
    Decrease,
    Both,
    None,
}

impl_from_str_as_str!(
    LaneChange,
    "increase" => Increase,
    "decrease" => Decrease,
    "both" => Both,
    "none" => None,
);

/// The known keywords for the simplified road mark type information
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum TypeSimplified {
    None,
    Solid,
    Broken,
    /// for double solid line
    SolidSolid,
    /// from inside to outside, exception: center lane – from left to right
    SolidBroken,
    /// from inside to outside, exception: center lane – from left to right
    BrokenSolid,
    /// from inside to outside, exception: center lane – from left to right
    BrokenBroken,
    BottsDots,
    /// meaning a grass edge
    Grass,
    Curb,
    /// if detailed description is given in child tags (via [`Type`])
    Custom,
    /// describing the limit of usable space on a road
    Edge,
}

impl_from_str_as_str!(
    TypeSimplified,
    "none" => None,
    "solid" => Solid,
    "broken" => Broken,
    "solid solid" => SolidSolid,
    "solid broken" => SolidBroken,
    "broken solid" => BrokenSolid,
    "broken broken" => BrokenBroken,
    "botts dots" => BottsDots,
    "grass" => Grass,
    "curb" => Curb,
    "custom" => Custom,
    "edge" => Edge,
);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Weight {
    Standard,
    Bold,
}

impl_from_str_as_str!(
    Weight,
    "standard" => Standard,
    "Bold" => Bold
);
