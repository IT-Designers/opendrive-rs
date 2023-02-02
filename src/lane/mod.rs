use crate::core::additional_data::AdditionalData;
use crate::lane::access::Access;
use crate::lane::border::Border;
use crate::lane::height::Height;
use crate::lane::material::Material;
use crate::lane::road_mark::RoadMark;
use crate::lane::rule::Rule;
use crate::lane::speed::Speed;
use crate::lane::width::Width;
use lane_choice::LaneChoice;
use lane_link::LaneLink;
use lane_type::LaneType;
use std::borrow::Cow;

pub mod access;
pub mod border;
pub mod center;
pub mod center_lane;
pub mod height;
pub mod lane_choice;
pub mod lane_link;
pub mod lane_section;
pub mod lane_type;
pub mod lanes;
pub mod left;
pub mod left_lane;
pub mod material;
pub mod offset;
pub mod predecessor_successor;
pub mod right;
pub mod right_lane;
pub mod road_mark;
pub mod rule;
pub mod speed;
pub mod type_link;
pub mod width;

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, PartialEq)]
pub struct Lane {
    pub link: Option<LaneLink>,
    pub choice: Vec<LaneChoice>,
    pub road_mark: Vec<RoadMark>,
    pub material: Vec<Material>,
    pub speed: Vec<Speed>,
    pub access: Vec<Access>,
    pub height: Vec<Height>,
    pub rule: Vec<Rule>,
    /// - `true` = keep lane on level, that is, do not apply superelevation;
    /// - `false` = apply superelevation to this lane (default, also used if attribute level is missing)
    pub level: Option<bool>,
    /// Type of the lane
    pub r#type: LaneType,
    pub additional_data: AdditionalData,
}

impl Lane {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "level" => self.level.map(|v| v.to_string()).as_deref(),
            "type" => Some(self.r#type.as_str()),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(link) = &self.link {
            visit_children!(visitor, "link" => link);
        }

        for choice in &self.choice {
            match choice {
                LaneChoice::Border(border) => visit_children!(visitor, "border" => border),
                LaneChoice::Width(width) => visit_children!(visitor, "width" => width),
            }
        }

        for road_mark in &self.road_mark {
            visit_children!(visitor, "roadMark" => road_mark);
        }

        for material in &self.material {
            visit_children!(visitor, "material" => material);
        }

        for speed in &self.speed {
            visit_children!(visitor, "speed" => speed);
        }

        for access in &self.access {
            visit_children!(visitor, "access" => access);
        }

        for height in &self.height {
            visit_children!(visitor, "height" => height);
        }

        for rule in &self.rule {
            visit_children!(visitor, "rule" => rule);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Lane
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut link = None;
        let mut choice = Vec::new();
        let mut road_mark = Vec::new();
        let mut material = Vec::new();
        let mut speed = Vec::new();
        let mut access = Vec::new();
        let mut height = Vec::new();
        let mut rule = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "link" => LaneLink => |v| link = Some(v),
            "border" => Border => |v| choice.push(LaneChoice::Border(v)),
            "width" => Width => |v| choice.push(LaneChoice::Width(v)),
            "roadMark" => RoadMark => |v| road_mark.push(v),
            "material" => Material => |v| material.push(v),
            "speed" => Speed => |v| speed.push(v),
            "access" => Access => |v| access.push(v),
            "height" => Height => |v| height.push(v),
            "rule" => Rule => |v| rule.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            link,
            choice,
            road_mark,
            material,
            speed,
            access,
            height,
            rule,
            level: read.attribute_opt("level")?,
            r#type: read.attribute("type")?,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Lane {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            link: u.arbitrary()?,
            choice: u.arbitrary()?,
            road_mark: u.arbitrary()?,
            material: u.arbitrary()?,
            speed: u.arbitrary()?,
            access: u.arbitrary()?,
            height: u.arbitrary()?,
            rule: u.arbitrary()?,
            level: u.arbitrary()?,
            r#type: u.arbitrary()?,
            additional_data: u.arbitrary()?,
        })
    }
}
