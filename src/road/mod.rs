use crate::junction::{ContactPoint, ElementDir};
use crate::road::geometry::PlanView;
use serde_derive::{Deserialize, Serialize};
use uom::si::f64::Length;

mod geometry;

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
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    #[serde(rename = "planView")]
    pub plan_view: Vec<PlanView>,
    // #[serde(rename = "elevationProfile")]
    // pub elevation_profile: Option<()>,
    // #[serde(rename = "lateralProfile")]
    // pub lateral_profile: Option<()>,
    // pub lanes: (),
    // pub objects: (),
    // pub signals: (),
    // pub surface: (),
    // pub raildroad: (),
}

/// Follows the road header if the road is linked to a successor or a predecessor. Isolated roads
/// may omit this element.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Link {
    pub predecessor: Option<PredecessorSuccessor>,
    pub successor: Option<PredecessorSuccessor>,
    // TODO pub additional_data: Vec<AdditionalData>,
}

/// Successors and predecessors can be junctions or roads. For each, different attribute sets shall
/// be used.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PredecessorSuccessor {
    /// Contact point of link on the linked element
    #[serde(rename = "contactPoint")]
    pub contact_point: Option<ContactPoint>,
    /// To be provided when elementS is used for the connection definition. Indicates the direction
    /// on the predecessor from which the road is entered.
    #[serde(rename = "elementDir")]
    pub element_dir: Option<ElementDir>,
    /// ID of the linked element
    #[serde(rename = "elementId")]
    pub element_id: String,
    /// Alternative to contactPoint for virtual junctions. Indicates a connection within the
    /// predecessor, meaning not at the start or end of the predecessor. Shall only be used for
    /// elementType "road"
    #[serde(rename = "elementS")]
    pub element_s: Option<Length>,
    /// Type of the linked element
    #[serde(rename = "elementType")]
    pub element_type: Vec<ElementType>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ElementType {
    #[serde(rename = "road")]
    Road,
    #[serde(rename = "junction")]
    Junction,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Rule {
    #[serde(rename = "RHT")]
    RightHandTraffic,
    #[serde(rename = "LHT")]
    LeftHandTraffic,
}
