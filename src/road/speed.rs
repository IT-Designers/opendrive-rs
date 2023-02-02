use crate::road::unit::SpeedUnit;
use std::borrow::Cow;

/// Defines the default maximum speed allowed in conjunction with the specified road type.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Speed {
    /// Maximum allowed speed. Given as string (only "no limit" / "undefined") or numerical value in
    /// the respective unit (see attribute unit). If the attribute unit is not specified, m/s is
    /// used as default.
    pub max: MaxSpeed,
    /// Unit of the attribute max. For values, see chapter “units”.
    pub unit: Option<SpeedUnit>,
}

impl Speed {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "max" => Some(&*self.max.as_str()),
            "unit" => self.unit.as_ref().map(SpeedUnit::as_str),
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

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Speed
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        read.expecting_no_child_elements_for(Self {
            max: read.attribute("max")?,
            unit: read.attribute_opt("unit")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MaxSpeed {
    Limit(f64),
    NoLimit,
    Undefined,
}

impl MaxSpeed {
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Self::Limit(limit) => Cow::Owned(limit.to_string()),
            Self::NoLimit => Cow::Borrowed("no limit"),
            Self::Undefined => Cow::Borrowed("undefined"),
        }
    }
}

impl core::str::FromStr for MaxSpeed {
    type Err = crate::parser::InvalidEnumValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if Self::NoLimit.as_str().eq_ignore_ascii_case(s) {
            Self::NoLimit
        } else if Self::Undefined.as_str().eq_ignore_ascii_case(s) {
            Self::Undefined
        } else if let Ok(limit) = s.parse::<f64>() {
            Self::Limit(limit)
        } else {
            return Err(crate::parser::InvalidEnumValue {
                r#type: core::any::type_name::<Self>().to_string(),
                value: s.to_string(),
            });
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for MaxSpeed {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(if u.arbitrary()? {
            Self::Limit(u.not_nan_f64()?)
        } else if u.arbitrary()? {
            Self::NoLimit
        } else {
            Self::Undefined
        })
    }
}
