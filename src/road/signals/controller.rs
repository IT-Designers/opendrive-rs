use std::borrow::Cow;
use vec1::Vec1;

/// Controllers provides identical states for one or more dynamic signals. Controllers serve as
/// wrappers for the behaviour of a group of signals. Controllers are used for dynamic speed control
/// on motorways, and to control traffic light switching phases.
#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    pub control: Vec1<Control>,
    /// Unique ID within database
    pub id: String,
    /// Name of the controller. May be chosen freely.
    pub name: Option<String>,
    /// Sequence number (priority) of this controller with respect to other controllers of same logical level
    pub sequence: Option<u64>,
}
impl Controller {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "id" => Some(self.id.as_str()),
            "name" => self.name.as_deref(),
            "sequence" => self.sequence.map(|s| s.to_string()).as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for control in &self.control {
            visit_children!(visitor, "control" => control);
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Controller
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut control = Vec::new();

        match_child_eq_ignore_ascii_case!(
            read,
            "control" true => Control => |v| control.push(v),
        );

        Ok(Self {
            control: Vec1::try_from_vec(control).unwrap(),
            id: read.attribute("id")?,
            name: read.attribute_opt("name")?,
            sequence: read.attribute_opt("sequence")?,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Controller {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::NotNan;
        Ok(Self {
            control: {
                let mut vec1 = Vec1::new(u.arbitrary()?);
                vec1.extend(u.arbitrary::<Vec<_>>()?);
                vec1
            },
            id: u.arbitrary()?,
            name: u.arbitrary()?,
            sequence: u.arbitrary()?,
        })
    }
}

/// Provides information about a single signal controlled by the corresponding controller.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Control {
    /// ID of the controlled signal
    pub signal_id: String,
    /// Type of control.
    /// Free Text, depends on the application.
    pub r#type: Option<String>,
}

impl Control {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "signalId" => Some(self.signal_id.as_str()),
            "type" => self.r#type.as_deref(),
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
impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Control
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            signal_id: read.attribute("signalId")?,
            r#type: read.attribute_opt("type")?,
        })
    }
}
