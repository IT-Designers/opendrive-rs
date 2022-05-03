use crate::core::additional_data::AdditionalData;
use crate::signal::signal_reference::SignalReference;
use crate::signal::Signal;
use std::borrow::Cow;

/// The `<signals>` element is the container for all signals along a road.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Signals {
    pub signal: Vec<Signal>,
    pub signal_reference: Vec<SignalReference>,
    pub additional_data: AdditionalData,
}

impl Signals {
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
        for signal in &self.signal {
            visit_children!(visitor, "signal" => signal);
        }

        for reference in &self.signal_reference {
            visit_children!(visitor, "signalReference" => reference);
        }

        self.additional_data.append_children(visitor)
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for Signals
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut signal = Vec::new();
        let mut signal_reference = Vec::new();
        let mut additional_data = AdditionalData::default();

        match_child_eq_ignore_ascii_case!(
            read,
            "signal" => Signal => |v| signal.push(v),
            "signalReference" => SignalReference => |v| signal_reference.push(v),
            _ => |_name, context| additional_data.fill(context),
        );

        Ok(Self {
            signal,
            signal_reference,
            additional_data,
        })
    }
}
