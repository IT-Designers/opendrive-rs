use crate::lane::access::restriction_type::AccessRestrictionType;
use rule::AccessRule;
use std::borrow::Cow;
use uom::si::f64::Length;
use uom::si::length::meter;

pub mod restriction_type;
pub mod rule;

/// Defines access restrictions for certain types of road users.
/// Each element is valid in direction of the increasing s coordinate until a new element is
/// defined. If multiple elements are defined, they shall be listed in ascending order.
#[derive(Debug, Clone, PartialEq)]
pub struct Access {
    /// Identifier of the participant to whom the restriction applies
    pub restriction: AccessRestrictionType,
    /// Specifies whether the participant given in the attribute @restriction is allowed or denied
    /// access to the given lane
    pub rule: Option<AccessRule>,
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    pub s_offset: Length,
}

impl Access {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "restriction" => Some(self.restriction.as_str()),
            "rule" => self.rule.as_ref().map(AccessRule::as_str),
            "sOffset" => Some(self.s_offset.value.to_scientific_string()).as_deref(),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Access
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            restriction: read.attribute("restriction")?,
            rule: read.attribute_opt("rule")?,
            s_offset: read.attribute("sOffset").map(Length::new::<meter>)?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Access {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            restriction: u.arbitrary()?,
            s_offset: Length::new::<meter>(u.not_nan_f64()?),
            rule: u.arbitrary()?,
        })
    }
}
