use crate::core::additional_data::AdditionalData;
use crate::object::border_type::BorderType;
use crate::object::corner_reference::CornerReference;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Specifies a border along certain outline points.
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub corner_reference: Vec<CornerReference>,
    /// ID of the outline to use
    pub outline_id: u64,
    /// Appearance of border
    pub r#type: BorderType,
    /// Use all outline points for border. “true” is used as default.
    pub use_complete_outline: Option<bool>,
    /// Border width
    pub width: Length,
    pub additional_data: AdditionalData,
}

impl Border {
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

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Border
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut corner_reference = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "cornerReference" => CornerReference => |v| corner_reference.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            outline_id: read.attribute("outlineId")?,
            r#type: read.attribute("type")?,
            use_complete_outline: read.attribute_opt("useCompleteOutline")?,
            width: read.attribute("width").map(Length::new::<meter>)?,
            corner_reference,
            additional_data,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Border {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            corner_reference: u.arbitrary()?,
            outline_id: u.arbitrary()?,
            r#type: u.arbitrary()?,
            use_complete_outline: u.arbitrary()?,
            width: Length::new::<meter>(u.not_nan_f64()?),
            additional_data: u.arbitrary()?,
        })
    }
}
