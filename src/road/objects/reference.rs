use crate::road::objects::validity::LaneValidity;
use crate::road::objects::Orientation;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// It is possible to link an object with one or more roads, signals or other objects using a
/// `<objectReference>` element. The referenced objects require a unique ID. The object reference
/// element consists of a main element and an optional lane validity element. The rules for validity
/// elements are the same as for objects.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectReference {
    /// Unique ID of the referred object within the database
    id: String,
    /// - "+" = valid in positive s-direction
    /// - "-" = valid in negative s-direction
    /// - "none" = valid in both directions
    orientation: Orientation,
    /// s-coordinate
    s: Length,
    /// t-coordinate
    t: Length,
    /// Validity of the object along s-axis (0.0 for point object)
    valid_length: Option<Length>,
    /// z offset relative to the elevation of the reference line
    z_offset: Option<Length>,
    validity: Vec<LaneValidity>,
}

impl ObjectReference {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut validity = Vec::new();

        find_map_parse_elem!(
            events,
            "validity" => |attributes| {
                validity.push(LaneValidity::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self {
            id: find_map_parse_attr!(attributes, "id", String)?,
            orientation: find_map_parse_attr!(attributes, "orientation", Orientation)?,
            s: find_map_parse_attr!(attributes, "s", f64).map(Length::new::<meter>)?,
            t: find_map_parse_attr!(attributes, "t", f64).map(Length::new::<meter>)?,
            valid_length: find_map_parse_attr!(attributes, "validLength", Option<f64>)?
                .map(Length::new::<meter>),
            z_offset: find_map_parse_attr!(attributes, "zOffset", Option<f64>)?
                .map(Length::new::<meter>),
            validity,
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
            "orientation" => Some(self.orientation.as_str()),
            "s" => Some(self.s.value.to_scientific_string()).as_deref(),
            "t" => Some(self.t.value.to_scientific_string()).as_deref(),
            "validLength" => self.valid_length.as_ref().map(|v| v.value.to_scientific_string()).as_deref(),
            "zOffset" => self.z_offset.as_ref().map(|v| v.value.to_scientific_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for validity in self.validity {
            visit_children!(visitor, "validity" => validity);
        }
        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for ObjectReference {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            id: u.arbitrary()?,
            orientation: u.arbitrary()?,
            s: Length::new::<meter>(u.not_nan_f64()?),
            t: Length::new::<meter>(u.not_nan_f64()?),
            valid_length: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            z_offset: u
                .arbitrary::<Option<()>>()?
                .map(|_| u.not_nan_f64())
                .transpose()?
                .map(Length::new::<meter>),
            validity: u.arbitrary()?,
        })
    }
}
