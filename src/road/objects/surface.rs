use std::borrow::Cow;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Used to describe the road surface elevation of an object.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Surface {
    pub crg: Option<Crg>,
}

impl Surface {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut crg = None;

        find_map_parse_elem!(
            events,
            "CRG" => |attributes| {
                crg = Some(Crg::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self { crg })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(visitor)
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        if let Some(crg) = &self.crg {
            visit_children!(visitor, "CRG" => crg);
        }
        Ok(())
    }
}

/// Elevation data described in {GLO_VAR_STA_ASAM_OpenCRG} are represented by the `<CRG>` element
/// within the `<surface>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Crg {
    /// Name of the file containing the CRG data.
    pub file: Option<String>,
    /// Determines if the object CRG hides the road surface CRG. Default is true.
    pub hide_road_surface_crg: Option<bool>,
    /// z-scale factor for the surface description (default = 1.0).
    pub z_scale: Option<f64>,
}

impl Crg {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            file: find_map_parse_attr!(attributes, "file", Option<String>)?,
            hide_road_surface_crg: find_map_parse_attr!(
                attributes,
                "hideRoadSurfaceCRG",
                Option<bool>
            )?,
            z_scale: find_map_parse_attr!(attributes, "zScale", Option<f64>)?,
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
            "file" => self.file.as_deref(),
            "hideRoadSurfaceCRG" => self.hide_road_surface_crg.map(|v| v.to_string()).as_deref(),
            "zScale" => self.z_scale.map(|v| v.to_scientific_string()).as_deref(),
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
impl arbitrary::Arbitrary<'_> for Crg {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            file: u.arbitrary()?,
            hide_road_surface_crg: u.arbitrary()?,
            z_scale: u
                .arbitrary::<Option<()>>()
                .map(|_| u.not_nan_f64())
                .transpose()?,
        })
    }
}
