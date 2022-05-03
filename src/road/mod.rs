use crate::core::additional_data::AdditionalData;
use crate::junction::contact_point::ContactPoint;
use crate::junction::element_dir::ElementDir;
use crate::lane::lanes::Lanes;
use crate::road::geometry::PlanView;
use crate::road::objects::surface::Surface;
use crate::road::objects::Objects;
use crate::road::profile::{ElevationProfile, LateralProfile};
use crate::road::r#type::Type;
use crate::road::railroad::Railroad;
use crate::road::signals::Signals;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

#[allow(deprecated)]
pub mod country_code;
pub mod geometry;
pub mod objects;
pub mod profile;
pub mod railroad;
pub mod signals;
pub mod speed;
pub mod r#type;
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
    pub r#type: Vec<Type>,
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
            "type" => Type => |v| r#type.push(v),
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

/// Follows the road header if the road is linked to a successor or a predecessor. Isolated roads
/// may omit this element.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Link {
    pub predecessor: Option<PredecessorSuccessor>,
    pub successor: Option<PredecessorSuccessor>,
    // TODO pub additional_data: Vec<AdditionalData>,
}
impl Link {
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
        if let Some(predecessor) = &self.predecessor {
            visit_children!(visitor, "predecessor" => predecessor);
        }

        if let Some(successor) = &self.successor {
            visit_children!(visitor, "successor" => successor);
        }

        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Link
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut predecessor = None;
        let mut successor = None;

        match_child_eq_ignore_ascii_case!(
            read,
            "predecessor" => PredecessorSuccessor => |v| predecessor = Some(v),
            "successor" => PredecessorSuccessor => |v| successor = Some(v),
        );

        Ok(Self {
            predecessor,
            successor,
        })
    }
}

/// Successors and predecessors can be junctions or roads. For each, different attribute sets shall
/// be used.
#[derive(Debug, Clone, PartialEq)]
pub struct PredecessorSuccessor {
    /// Contact point of link on the linked element
    pub contact_point: Option<ContactPoint>,
    /// To be provided when elementS is used for the connection definition. Indicates the direction
    /// on the predecessor from which the road is entered.
    pub element_dir: Option<ElementDir>,
    /// ID of the linked element
    pub element_id: String,
    /// Alternative to contactPoint for virtual junctions. Indicates a connection within the
    /// predecessor, meaning not at the start or end of the predecessor. Shall only be used for
    /// elementType "road"
    pub element_s: Option<Length>,
    /// Type of the linked element
    pub element_type: Option<ElementType>,
}

impl PredecessorSuccessor {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "contactPoint" => self.contact_point.as_ref().map(ContactPoint::as_str),
            "elementDir" => self.element_dir.as_ref().map(ElementDir::as_str),
            "elementId" => Some(self.element_id.as_str()),
            "elementS" => self.element_s.map(|v| v.value.to_scientific_string()).as_deref(),
            "elementType" => self.element_type.as_ref().map(ElementType::as_str),
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

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            contact_point: read.attribute_opt("contactPoint")?,
            element_dir: read.attribute_opt("elementDir")?,
            element_id: read.attribute("elementId")?,
            element_s: read.attribute_opt("elementS")?.map(Length::new::<meter>),
            element_type: read.attribute_opt("elementType")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for PredecessorSuccessor {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            contact_point: u.arbitrary()?,
            element_dir: u.arbitrary()?,
            element_id: u.arbitrary()?,
            element_s: if u.arbitrary()? {
                Some(Length::new::<meter>(u.not_nan_f64()?))
            } else {
                None
            },
            element_type: u.arbitrary()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum ElementType {
    Road,
    Junction,
}

impl_from_str_as_str!(
    ElementType,
    "road" => Road,
    "junction" => Junction,
);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Rule {
    RightHandTraffic,
    LeftHandTraffic,
}

impl_from_str_as_str!(
    Rule,
    "RHT" => RightHandTraffic,
    "LHT" => LeftHandTraffic,
);
