use crate::core::additional_data::AdditionalData;
use crate::lane::lanes::Lanes;
use crate::object::objects::Objects;
use crate::railroad::Railroad;
use crate::road::profile::ElevationProfile;
use crate::road::road_type::RoadType;
use crate::road::surface::Surface;
use crate::signal::Signals;
use geometry::plan_view::PlanView;
use link::Link;
use profile::lateral_profile::LateralProfile;
use rule::Rule;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

#[allow(deprecated)]
pub mod country_code;
pub mod crg;
pub mod element_type;
pub mod geometry;
pub mod link;
pub mod predecessor_successor;
pub mod profile;
pub mod road_type;
pub mod road_type_e;
pub mod rule;
pub mod speed;
pub mod surface;
pub mod unit;

/// In ASAM OpenDRIVE, the road network is represented by `<road>` elements. Each road runs along
/// one road reference line. A road shall have at least one lane with a width larger than 0.
/// Vehicles may drive in both directions of the reference line. The standard driving direction is
/// defined by the value which is assigned to the @rule attribute (RHT=right-hand traffic,
/// LHT=left-hand traffic).
/// ASAM OpenDRIVE roads may be roads in the real road network or artificial road network created
/// for application use. Each road is described by one or more `<road>` elements. One `<road>`
/// element may cover a long stretch of a road, shorter stretches between junctions, or even several
/// roads. A new `<road>` element should only start if the properties of the road cannot be
/// described within the previous `<road>` element or if a junction is required.
#[derive(Debug, Clone, PartialEq)]
pub struct Road {
    /// Unique ID within the database. If it represents an integer number, it should comply to
    /// `uint32_t` and stay within the given range.
    pub id: String,
    /// ID of the junction to which the road belongs as a connecting road (= -1 for none)
    pub junction: String,
    /// Total length of the reference line in the xy-plane. Change in length due to elevation is not
    /// considered.
    /// Only positive values are valid.
    pub length: Length,
    /// Name of the road. May be chosen freely.
    pub name: Option<String>,
    /// Basic rule for using the road; RHT=right-hand traffic, LHT=left-hand traffic. When this
    /// attribute is missing, RHT is assumed.
    pub rule: Option<Rule>,
    pub link: Option<Link>,
    pub r#type: Vec<RoadType>,
    pub plan_view: PlanView,
    pub elevation_profile: Option<ElevationProfile>,
    pub lateral_profile: Option<LateralProfile>,
    pub lanes: Lanes,
    pub objects: Option<Objects>,
    pub signals: Option<Signals>,
    pub surface: Option<Surface>,
    pub railroad: Option<Railroad>,
    pub additional_data: AdditionalData,
}

impl Road {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "id" => Some(self.id.as_str()),
            "junction" => Some(self.junction.as_str()),
            "length" => Some(self.length.value.to_scientific_string()).as_deref(),
            "name" => self.name.as_deref(),
            "rule" => self.rule.as_ref().map(Rule::as_str),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(link) = &self.link {
            visit_children!(visitor, "link" => link);
        }

        for r#type in &self.r#type {
            visit_children!(visitor, "type" => r#type);
        }

        if let Some(elevation) = &self.elevation_profile {
            visit_children!(visitor, "elevationProfile" => elevation);
        }

        if let Some(lateral) = &self.lateral_profile {
            visit_children!(visitor, "lateralProfile" => lateral);
        }

        if let Some(objects) = &self.objects {
            visit_children!(visitor, "objects" => objects);
        }

        if let Some(signals) = &self.signals {
            visit_children!(visitor, "signals" => signals);
        }

        if let Some(surface) = &self.surface {
            visit_children!(visitor, "surface" => surface);
        }

        if let Some(railroad) = &self.railroad {
            visit_children!(visitor, "railroad" => railroad);
        }

        visit_children!(
            visitor,
            "planView" => self.plan_view,
            "lanes" => self.lanes,
        );

        self.additional_data.append_children(&mut visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Road
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut link = None;
        let mut r#type = Vec::new();
        let mut plan_view = None;
        let mut elevation_profile = None;
        let mut lateral_profile = None;
        let mut lanes = None;
        let mut objects = None;
        let mut signals = None;
        let mut surface = None;
        let mut railroad = None;
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "link" => Link => |v| link = Some(v),
            "type" => RoadType => |v| r#type.push(v),
            "planView" true => PlanView => |v| plan_view = Some(v),
            "elevationProfile" => ElevationProfile => |v| elevation_profile = Some(v),
            "lateralProfile" => LateralProfile => |v| lateral_profile = Some(v),
            "lanes" true => Lanes => |v| lanes = Some(v),
            "objects" => Objects => |v| objects = Some(v),
            "signals" => Signals => |v| signals = Some(v),
            "surface" => Surface => |v| surface = Some(v),
            "railroad" => Railroad => |v| railroad = Some(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            id: read.attribute("id")?,
            junction: read.attribute("junction")?,
            length: read.attribute("length").map(Length::new::<meter>)?,
            name: read.attribute_opt("name")?,
            rule: read.attribute_opt("rule")?,
            link,
            r#type,
            plan_view: plan_view.unwrap(),
            elevation_profile,
            lateral_profile,
            lanes: lanes.unwrap(),
            objects,
            signals,
            surface,
            railroad,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Road {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            id: u.arbitrary()?,
            junction: u.arbitrary()?,
            length: Length::new::<meter>(u.not_nan_f64()?),
            name: u.arbitrary()?,
            rule: u.arbitrary()?,
            link: u.arbitrary()?,
            r#type: u.arbitrary()?,
            plan_view: u.arbitrary()?,
            elevation_profile: u.arbitrary()?,
            lateral_profile: u.arbitrary()?,
            lanes: u.arbitrary()?,
            objects: u.arbitrary()?,
            signals: u.arbitrary()?,
            surface: u.arbitrary()?,
            railroad: u.arbitrary()?,
            additional_data: u.arbitrary()?,
        })
    }
}
