use crate::junction::{ContactPoint, ElementDir};
use crate::road::geometry::PlanView;
use crate::road::lane::Lanes;
use crate::road::profile::{ElevationProfile, LateralProfile};
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

pub mod geometry;
pub mod lane;
pub mod profile;

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
    pub plan_view: PlanView,
    pub elevation_profile: Option<ElevationProfile>,
    pub lateral_profile: Option<LateralProfile>,
    pub lanes: Lanes,
    // pub objects: (),
    // pub signals: (),
    // pub surface: (),
    // pub raildroad: (),
}
impl Road {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut link = None;
        let mut plan_view = None;
        let mut elevation_profile = None;
        let mut lateral_profile = None;
        let mut lanes = None;

        find_map_parse_elem!(
            events,
            "link" => |attributes| {
                link = Some(Link::from_events(events, attributes)?);
                Ok(())
            },
            "planView" true => |attributes| {
                plan_view = Some(PlanView::from_events(events, attributes)?);
                Ok(())
            },
            "elevationProfile" => |attributes| {
                elevation_profile = Some(ElevationProfile::from_events(events, attributes)?);
                Ok(())
            },
            "lateralProfile" => |attributes| {
                lateral_profile = Some(LateralProfile::from_events(events, attributes)?);
                Ok(())
            },
            "lanes" true => |attributes| {
                lanes = Some(Lanes::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self {
            id: find_map_parse_attr!(attributes, "id", String)?,
            junction: find_map_parse_attr!(attributes, "junction", String)?,
            length: find_map_parse_attr!(attributes, "length", f64).map(Length::new::<meter>)?,
            name: find_map_parse_attr!(attributes, "name", Option<String>)?,
            rule: find_map_parse_attr!(attributes, "rule", Option<Rule>)?,
            link,
            plan_view: plan_view.unwrap(),
            elevation_profile,
            lateral_profile,
            lanes: lanes.unwrap(),
        })
    }

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

        if let Some(elevation) = &self.elevation_profile {
            visit_children!(visitor, "elevationProfile" => elevation);
        }

        if let Some(lateral) = &self.lateral_profile {
            visit_children!(visitor, "lateralProfile" => lateral);
        }

        visit_children!(
            visitor,
            "planView" => self.plan_view,
            "lanes" => self.lanes,
        );

        Ok(())
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
            plan_view: u.arbitrary()?,
            elevation_profile: u.arbitrary()?,
            lateral_profile: u.arbitrary()?,
            lanes: u.arbitrary()?,
        })
    }
}

/// Follows the road header if the road is linked to a successor or a predecessor. Isolated roads
/// may omit this element.
#[derive(Default, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Link {
    pub predecessor: Option<PredecessorSuccessor>,
    pub successor: Option<PredecessorSuccessor>,
    // TODO pub additional_data: Vec<AdditionalData>,
}
impl Link {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut this = Self::default();

        find_map_parse_elem!(
            events,
            "predecessor" => |attributes| {
                this.predecessor = Some(PredecessorSuccessor::from_events(events, attributes)?);
                Ok(())
            },
            "successor" => |attributes| {
                this.successor = Some(PredecessorSuccessor::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(this)
    }

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
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            contact_point: find_map_parse_attr!(attributes, "contactPoint", Option<ContactPoint>)?,
            element_dir: find_map_parse_attr!(attributes, "elementDir", Option<ElementDir>)?,
            element_id: find_map_parse_attr!(attributes, "elementId", String)?,
            element_s: find_map_parse_attr!(attributes, "elementS", Option<f64>)?
                .map(Length::new::<meter>),
            element_type: find_map_parse_attr!(attributes, "elementType", Option<ElementType>)?,
        })
    }

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
