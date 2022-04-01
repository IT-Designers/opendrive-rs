use std::convert::Infallible;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("XML parsing failed: {0}")]
    XmlError(#[from] xml::reader::Error),
    #[error("The required attribute `{0} is missing")]
    AttributeMissing(String),
    #[error("The required element `{0} is missing")]
    ElementMissing(String),
    #[error("A child element in `{0} is missing")]
    ChildElementIsMissing(String),
    #[error("Failed to parse value for attribute `{0}`: {1}")]
    ParseError(String, ParseError),
    #[error("Invalid value for `{name}`: {value}")]
    InvalidValueFor { name: String, value: String },
}

impl Error {
    pub fn child_missing<T: ?Sized>() -> Self {
        Self::ChildElementIsMissing(core::any::type_name::<T>().to_string())
    }

    pub fn invalid_value_for<T: ?Sized, V: Into<String>>(value: V) -> Self {
        Self::InvalidValueFor {
            name: core::any::type_name::<T>().to_string(),
            value: value.into(),
        }
    }
}

impl From<(&str, Infallible)> for Error {
    fn from(_: (&str, Infallible)) -> Self {
        unreachable!()
    }
}

impl From<(&str, Error)> for Error {
    fn from((_, error): (&str, Error)) -> Self {
        error
    }
}

impl From<(&str, ParseIntError)> for Error {
    fn from((name, error): (&str, ParseIntError)) -> Self {
        Self::ParseError(name.to_string(), ParseError::from(error))
    }
}

impl From<(&str, ParseFloatError)> for Error {
    fn from((name, error): (&str, ParseFloatError)) -> Self {
        Self::ParseError(name.to_string(), ParseError::from(error))
    }
}

impl From<(&str, ParseBoolError)> for Error {
    fn from((name, error): (&str, ParseBoolError)) -> Self {
        Self::ParseError(name.to_string(), ParseError::from(error))
    }
}

#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum ParseError {
    Int(ParseIntError),
    Float(ParseFloatError),
    Bool(ParseBoolError),
}

macro_rules! find_map_parse_attr {
    ($attrs:ident, $name:literal, Option<$ty:ty>) => {
        $attrs
            .iter()
            .find(|a| a.name.local_name == $name)
            .map(|a| {
                a.value
                    .parse::<$ty>()
                    .map_err(|e| crate::parser::Error::from((stringify!($ty), e)))
            })
            .transpose()
    };
    ($attrs:ident, $name:literal, $ty:ty) => {
        find_map_parse_attr!($attrs, $name, Option<$ty>).and_then(|v| {
            v.ok_or_else(|| crate::parser::Error::AttributeMissing($name.to_string()))
        })
    };
}

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
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace: _,
                } => {
                    let mut __index = 1;
                    $(
                        if name.local_name == $name {
                            let v: Result<(), crate::parser::Error> = $body(attributes);
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
                            XmlEvent::StartElement { .. } => depth += 1,
                            XmlEvent::EndElement { .. } => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                XmlEvent::EndElement { .. } => break,
                _event => {
                    $(
                        let v: Result<(), crate::parser::Error> = $alt(_event);
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
                    return Err(crate::parser::Error::ElementMissing(stringify!($name).to_string()))
                )?
            }
            __index += 1;
        )*
    }
}
