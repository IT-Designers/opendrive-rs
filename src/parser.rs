use backtrace::Backtrace;
use std::convert::Infallible;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("XML parsing failed: {0}")]
    XmlError(#[from] xml::reader::Error),
    #[error("The required attribute `{name} is missing")]
    AttributeMissing {
        name: String,
        backtrace: Box<Backtrace>,
    },
    #[error("The required element `{name} is missing")]
    ElementMissing {
        name: String,
        backtrace: Box<Backtrace>,
    },
    #[error("A child element in `{0} is missing")]
    ChildElementIsMissing(String, Box<Backtrace>),
    #[error("Failed to parse value of field `{field}` to `{ty}`: {error}")]
    ParseError {
        path: String,
        field: String,
        ty: String,
        error: ParseError,
        bt: Box<Backtrace>,
    },
    #[error("Invalid value for `{name}`: {value}")]
    InvalidValueFor { name: String, value: String },
}

impl Error {
    #[inline]
    pub fn child_missing<T: ?Sized>() -> Self {
        Self::ChildElementIsMissing(
            core::any::type_name::<T>().to_string(),
            Box::new(Backtrace::new()),
        )
    }

    pub fn invalid_value_for<T: ?Sized, V: Into<String>>(value: V) -> Self {
        Self::InvalidValueFor {
            name: core::any::type_name::<T>().to_string(),
            value: value.into(),
        }
    }

    #[inline]
    pub fn missing_attribute(attribute_name: impl Into<String>) -> Self {
        Self::AttributeMissing {
            name: attribute_name.into(),
            backtrace: Box::new(Backtrace::new()),
        }
    }

    #[inline]
    pub fn missing_element(element_name: impl Into<String>) -> Self {
        Self::ElementMissing {
            name: element_name.into(),
            backtrace: Box::new(Backtrace::new()),
        }
    }

    #[inline]
    pub fn parser_failed(
        field: impl Into<String>,
        ty: impl Into<String>,
        error: impl Into<ParseError>,
    ) -> Self {
        Self::ParseError {
            path: String::new(),
            field: field.into(),
            ty: ty.into(),
            error: error.into(),
            bt: Box::new(Backtrace::new()),
        }
    }
}

impl From<(&str, &str, Error)> for Error {
    #[inline]
    fn from((_field, _ty, error): (&str, &str, Error)) -> Self {
        error
    }
}

impl<T> From<(&str, &str, T)> for Error
where
    T: Into<ParseError>,
{
    #[inline]
    fn from((field, ty, error): (&str, &str, T)) -> Self {
        Self::parser_failed(field, ty, error)
    }
}

#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum ParseError {
    Int(ParseIntError),
    Float(ParseFloatError),
    Bool(ParseBoolError),
}

impl From<Infallible> for ParseError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

pub trait ToScientificString {
    fn to_scientific_string(&self) -> String;
}

impl ToScientificString for f64 {
    fn to_scientific_string(&self) -> String {
        // TODO {:.17e+02} does not work
        format!("{:.17e}", self)
    }
}

#[macro_export]
macro_rules! find_map_parse_attr {
    ($attrs:ident, $name:literal, Option<$ty:ty>) => {
        $attrs
            .iter()
            .find(|a| a.name.local_name == $name)
            .map(|a| {
                a.value
                    .parse::<$ty>()
                    .map_err(|e| $crate::parser::Error::from(($name, stringify!($ty), e)))
            })
            .transpose()
    };
    ($attrs:ident, $name:literal, $ty:ty) => {
        find_map_parse_attr!($attrs, $name, Option<$ty>).and_then(|v| {
            v.ok_or_else(|| $crate::parser::Error::missing_attribute($name.to_string()))
        })
    };
}

#[macro_export]
macro_rules! find_map_parse_elem {
    ($events:ident $(, $name:literal $($req:literal)? => $body:expr)* $(, _ => $alt:expr)? $(,)?) => {
        let mut __fields = [
            true,
            $(
                {
                    #[allow(unused_mut, unused_assignments)]
                    let mut r = false;
                    $(r = $req;)?
                    r
                },
            )*
        ];

        while let Some(event) = $events.next() {
            match event? {
                xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace: _,
                } => {
                    let mut __index = 1;
                    $(
                        if name.local_name == $name {
                            let v: Result<(), $crate::parser::Error> = $body(attributes);
                            v?;
                            __fields[__index] = false;
                            continue;
                        }
                        __index += 1;
                    )*

                    // none captured, need to skip to element end
                    dbg!(&name.local_name, &attributes);
                    let mut depth = 1_usize;
                    while let Some(event) = $events.next() {
                        match event? {
                            xml::reader::XmlEvent::StartElement { .. } => depth += 1,
                            xml::reader::XmlEvent::EndElement { .. } => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                xml::reader::XmlEvent::EndElement { .. } => break,
                _event => {
                    $(
                        let v: Result<(), $crate::parser::Error> = $alt(_event);
                        v?;
                    )?
                }
            }
        }

        let mut __index = 1;
        $(
            let _ = $name;
            if __fields[__index] {
                $(
                    let _: bool = $req;
                    return Err($crate::parser::Error::missing_element($name.to_string()))
                )?
            }
            __index += 1;
        )*
    }
}

#[macro_export]
macro_rules! visit_attributes {
    ($visitor:ident$(, $name:literal => $attr:expr)* $(,)?) => {
        $visitor(Cow::Borrowed({
            #[allow(unused)]
            use $crate::parser::ToScientificString as _;
            &[
                $(
                    xml::attribute::Attribute::new(
                        xml::name::Name::local($name),
                        $attr
                    ),
                )*
            ]
        }))
    }
}

#[macro_export]
macro_rules! visit_attributes_flatten {
    ($visitor:ident$(, $name:literal => $attr:expr)* $(,)?) => {
        $visitor(Cow::Borrowed({
            #[allow(unused)]
            use $crate::parser::ToScientificString as _;
            &[
                $(
                    $attr.map(|attr| ($name, attr)),
                )*
            ].into_iter().flatten().map(|(name, attr)| {
                xml::attribute::Attribute::new(
                    xml::name::Name::local(name),
                    attr
                )
            }).collect::<Vec<_>>()
        }))
    }
}

#[macro_export]
macro_rules! visit_children {
    ($visitor:ident $(, $name:literal => $child:expr)* $(,)?) => {
        {
            let _ = &mut $visitor;
            $(
                $child.visit_attributes(|attributes| {
                    $visitor(xml::writer::XmlEvent::StartElement {
                        name: xml::name::Name::local($name),
                        attributes,
                        namespace: std::borrow::Cow::Owned(xml::namespace::Namespace::empty()),
                    })
                })?;
                $child.visit_children(&mut $visitor)?;
                $visitor(xml::writer::XmlEvent::EndElement { name: None })?;
            )*
        }
    }
}

#[macro_export]
macro_rules! impl_from_str_as_str {
    ($ty:ty $(, $name:literal => $value:ident)* $(,)?) => {
        impl $ty {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(<$ty>::$value => $name,)*
                }
            }
        }

        impl core::str::FromStr for $ty {
            type Err = $crate::parser::Error;

            #[allow(deprecated)]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(_ if s.eq_ignore_ascii_case(Self::$value.as_str()) => Ok(Self::$value),)*
                    _ => Err($crate::parser::Error::invalid_value_for::<Self, _>(s)),
                }
            }
        }
    };
}
