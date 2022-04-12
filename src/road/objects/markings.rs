use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

/// Describes the appearance of the parking space with multiple marking elements.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Markings {
    pub marking: Vec<Marking>,
}

impl Markings {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut marking = Vec::new();

        find_map_parse_elem!(
            events,
            "marking" true => |attributes| {
                marking.push(Marking::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self { marking })
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
        for marking in &self.marking {
            visit_children!(visitor, "marking" => marking);
        }
        Ok(())
    }
}

/// Specifies a marking that is either attached to one side of the object bounding box or
/// referencing outline points.
#[derive(Debug, Clone, PartialEq)]
pub struct Marking {
    /// ID of the outline to use
    pub outline_id: u64,
    /// Appearance of border
    pub r#type: BorderType,
    /// Use all outline points for border. “true” is used as default.
    pub use_complete_outline: Option<bool>,
    /// Border width
    pub width: Length,
    pub corner_reference: Vec<CornerReference>,
}

impl Marking {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut corner_reference = Vec::new();

        find_map_parse_elem!(
            events,
            "cornerReference" => |attributes| {
                corner_reference.push(CornerReference::from_events(events, attributes)?);
                Ok(())
            },
        );

        Ok(Self {
            outline_id: find_map_parse_attr!(attributes, "outlineId", u64)?,
            r#type: find_map_parse_attr!(attributes, "type", BorderType)?,
            use_complete_outline: find_map_parse_attr!(
                attributes,
                "useCompleteOutline",
                Option<bool>
            )?,
            width: find_map_parse_attr!(attributes, "width", f64).map(Length::new::<meter>)?,
            corner_reference,
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
            "outlineId" => Some(self.outline_id.to_string()).as_deref(),
            "type" => Some(self.r#type.as_str()),
            "useCompleteOutline" => self.use_complete_outline.map(|v| v.to_string()).as_deref(),
            "width" => Some(self.width.value.to_scientific_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for corner_reference in &self.corner_reference {
            visit_children!(visitor, "cornerReference" => corner_reference);
        }
        Ok(())
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Marking {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            corner_reference: u.arbitrary()?,
            outline_id: u.arbitrary()?,
            r#type: u.arbitrary()?,
            use_complete_outline: u.arbitrary()?,
            width: Length::new::<meter>(u.not_nan_f64()?),
        })
    }
}

/// Specifies a point by referencing an existing outline point.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct CornerReference {
    /// Identifier of the referenced outline point
    pub id: u64,
}

impl CornerReference {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);

        Ok(Self {
            id: find_map_parse_attr!(attributes, "id", u64)?,
        })
    }

    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes!(
            visitor,
            "id" => &self.id.to_string(),
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum BorderType {
    Concrete,
    Curb,
}

impl_from_str_as_str!(
    BorderType,
    "concrete" => Concrete,
    "curb" => Curb,
);
