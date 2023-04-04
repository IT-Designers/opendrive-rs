use std::borrow::Cow;
use std::collections::HashMap;

/// Ancillary data should be described near the element it refers to. Ancillary data contains data
/// that are not yet described in ASAM OpenDRIVE, or data that is needed by an application for a
/// specific reason. Examples are different road textures.
/// In ASAM OpenDRIVE, ancillary data is represented by `<userData>` elements. They may be stored at
/// any element in ASAM OpenDRIVE.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct UserData {
    /// Code for the user data. Free text, depending on application.
    pub code: String,
    /// User data. Free text, depending on application.
    pub value: Option<String>,
    pub elements: Vec<Element>,
}

impl UserData {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "code" => Some(self.code.as_str()),
            "value" => self.value.as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        for element in &self.elements {
            element.visit(&mut visitor)?;
        }
        Ok(())
    }
}

impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for UserData
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(mut read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        let mut elements = Vec::new();

        read.children(|name, context| {
            elements.push(Element::try_from((name.to_string(), context))?);
            Ok(())
        })?;

        Ok(Self {
            code: read.attribute("code")?,
            value: read.attribute_opt("value")?,
            elements,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Element>,
}

impl Element {
    pub fn visit(
        &self,
        // prevent recursion limit overflows on nightly with fuzzer by passing &mut impl...
        visitor: &mut impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visitor(xml::writer::XmlEvent::StartElement {
            name: xml::name::Name::local(&self.name),
            attributes: Cow::Owned(
                self.attributes
                    .iter()
                    .map(|(key, value)| {
                        xml::attribute::Attribute::new(xml::name::Name::local(key), value)
                    })
                    .collect::<Vec<_>>(),
            ),
            namespace: std::borrow::Cow::Owned(xml::namespace::Namespace::empty()),
        })?;
        for child in &self.children {
            child.visit(visitor)?;
        }
        visitor(xml::writer::XmlEvent::EndElement { name: None })?;
        Ok(())
    }
}

impl<'a, I> TryFrom<(String, crate::parser::ReadContext<'a, I>)> for Element
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = Box<crate::parser::Error>;

    fn try_from(
        (name, mut read): (String, crate::parser::ReadContext<'a, I>),
    ) -> Result<Self, Self::Error> {
        let mut children = Vec::new();

        read.children(|name, context| {
            children.push(Element::try_from((name.to_string(), context))?);
            Ok(())
        })?;

        Ok(Self {
            name,
            attributes: read
                .attributes()
                .map(|a| (a.name.local_name.to_string(), a.value.clone()))
                .collect(),
            children,
        })
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for Element {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::ArbitraryStrings;
        Ok(Self {
            name: u.arbitrary_string(1..=10, &['a'..='z', 'A'..='Z'])?,
            attributes: (0..u.arbitrary_len::<HashMap<String, String>>()?)
                .map(|_| {
                    (|| {
                        Ok((
                            u.arbitrary_string(1..=10, &['a'..='z', 'A'..='Z'])?,
                            u.arbitrary_string(1..=10, &['a'..='z', 'A'..='Z'])?,
                        ))
                    })()
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
            children: u.arbitrary()?,
        })
    }
}
