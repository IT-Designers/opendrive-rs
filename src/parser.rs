use backtrace::Backtrace;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::{FromStr, ParseBoolError};
use xml::attribute::OwnedAttribute;

pub type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Debug, Copy, Clone)]
pub struct Path<'a> {
    pub parent: Option<&'a Path<'a>>,
    pub name: &'a str,
}

impl Display for Path<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = self.parent {
            write!(f, "{}.{}", parent, self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

pub struct ReadContext<'a, I>
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    iterator: &'a mut I,
    path: Path<'a>,
    attributes: Vec<OwnedAttribute>,
    children_done: bool,
    #[cfg(debug_assertions)]
    read_attributes: std::cell::RefCell<Vec<String>>,
}

impl<'a, I> ReadContext<'a, I>
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    pub fn from_parent(
        iterator: &'a mut I,
        path: Path<'a>,
        attributes: Vec<OwnedAttribute>,
    ) -> Self {
        Self::from(iterator)
            .with_path(path)
            .with_attributes(attributes)
    }

    pub fn with_path(mut self, path: Path<'a>) -> Self {
        self.path = path;
        self
    }

    pub fn with_attributes(mut self, attributes: Vec<OwnedAttribute>) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn path(&self) -> Path {
        self.path
    }

    pub fn element_name(&self) -> &str {
        self.path.name
    }

    pub fn attribute<T: FromStr>(&self, name: &str) -> Result<T>
    where
        T::Err: Into<ParseError>,
    {
        #[cfg(debug_assertions)]
        self.read_attributes.borrow_mut().push(name.to_string());
        for attribute in &self.attributes {
            if attribute.name.local_name.eq_ignore_ascii_case(name) {
                return match T::from_str(&attribute.value) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Box::new(Error::ParseError {
                        path: self.path.to_string(),
                        field: name.to_string(),
                        ty: core::any::type_name::<T>().to_string(),
                        error: e.into(),
                        bt: Box::new(Backtrace::new()),
                    })),
                };
            }
        }
        Err(Box::new(Error::missing_attribute(
            self.path.to_string(),
            name,
            core::any::type_name::<T>(),
        )))
    }

    pub fn attribute_opt<T: FromStr>(&self, name: &str) -> Result<Option<T>>
    where
        T::Err: Into<ParseError>,
    {
        #[cfg(debug_assertions)]
        self.read_attributes.borrow_mut().push(name.to_string());
        for attribute in &self.attributes {
            if attribute.name.local_name.eq_ignore_ascii_case(name) {
                return match T::from_str(&attribute.value) {
                    Ok(v) => Ok(Some(v)),
                    Err(e) => Err(Box::new(Error::ParseError {
                        path: self.path.to_string(),
                        field: name.to_string(),
                        ty: core::any::type_name::<T>().to_string(),
                        error: e.into(),
                        bt: Box::new(Backtrace::new()),
                    })),
                };
            }
        }
        Ok(None)
    }

    pub fn attributes(&self) -> impl Iterator<Item = &OwnedAttribute> {
        #[allow(clippy::map_identity)] // because of debug assertions cfg flag
        self.attributes.iter().map(|a| {
            #[cfg(debug_assertions)]
            self.read_attributes
                .borrow_mut()
                .push(a.name.local_name.clone());
            a
        })
    }

    #[allow(clippy::type_complexity)] // for now, getting removed later on most properly anyway...
    pub fn elements(
        &mut self,
        mapper: &mut [(
            &str,
            &mut dyn for<'b> FnMut(&'b mut ReadContext<'_, I>) -> Result<()>,
        )],
    ) -> Result<()> {
        'outer: while let Some(event) = self.iterator.next() {
            match event.map_err(Error::from).map_err(Box::new)? {
                xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace: _,
                } => {
                    let mut context = ReadContext::from_parent(
                        &mut *self.iterator,
                        Path {
                            parent: Some(&self.path),
                            name: &name.local_name,
                        },
                        attributes,
                    );
                    for (mapper_name, mapper_fn) in mapper.iter_mut() {
                        if name.local_name.eq_ignore_ascii_case(mapper_name) {
                            mapper_fn(&mut context)?;
                            continue 'outer;
                        }
                    }
                    context.elements(&mut [])?;
                }
                xml::reader::XmlEvent::EndElement { name } => {
                    debug_assert_eq!(self.element_name(), &name.local_name);
                    self.children_done = true;
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    }

    #[inline]
    pub fn children(
        &mut self,
        mapper: impl for<'b> FnMut(&'b str, ReadContext<'_, I>) -> Result<()>,
    ) -> Result<()> {
        self.children_or_cdata(mapper, |_| Ok(()))
    }

    pub fn children_or_cdata(
        &mut self,
        mut mapper: impl for<'b> FnMut(&'b str, ReadContext<'_, I>) -> Result<()>,
        mut cdata: impl for<'b> FnMut(String) -> Result<()>,
    ) -> Result<()> {
        while let Some(event) = self.iterator.next() {
            match event.map_err(Error::from).map_err(Box::new)? {
                xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace: _,
                } => {
                    if let Err(e) = mapper(
                        &name.local_name,
                        ReadContext::from_parent(
                            &mut *self.iterator,
                            Path {
                                parent: Some(&self.path),
                                name: &name.local_name,
                            },
                            attributes,
                        ),
                    ) {
                        // dont walk any more elements on an error, just drop them
                        self.children_done = true;
                        return Err(e);
                    }
                }
                xml::reader::XmlEvent::EndElement { name } => {
                    debug_assert_eq!(self.element_name(), &name.local_name);
                    self.children_done = true;
                    break;
                }
                xml::reader::XmlEvent::EndDocument => {
                    debug_assert!(self.path.parent.is_none());
                    debug_assert!(self.path.name.is_empty());
                    self.children_done = true;
                    break;
                }
                xml::reader::XmlEvent::CData(data) => {
                    cdata(data)?;
                }
                other => {
                    drop(other);
                }
            }
        }
        Ok(())
    }

    #[inline]
    pub fn expecting_no_child_elements(&mut self) -> Result<()> {
        self.children(|name, mut read| {
            dbg!(name);
            read.expecting_no_child_elements()
        })
    }

    #[inline]
    pub fn expecting_no_child_elements_for<T>(&mut self, value: T) -> Result<T> {
        self.children(|_name, mut read| {
            dbg!(read.path().to_string());
            read.expecting_no_child_elements()
        })?;
        Ok(value)
    }
}

impl<'a, I> From<&'a mut I> for ReadContext<'a, I>
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    fn from(iterator: &'a mut I) -> Self {
        Self {
            iterator,
            path: Path {
                parent: None,
                name: "",
            },
            attributes: Vec::new(),
            children_done: false,
            #[cfg(debug_assertions)]
            read_attributes: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl<'a, I> Drop for ReadContext<'a, I>
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    fn drop(&mut self) {
        if !self.children_done {
            let _ = self.children(|name, ctx| {
                // walk it by dropping it
                let _ = (name, ctx);
                Ok(())
            });
        }

        #[cfg(debug_assertions)]
        {
            dbg!(self.path().to_string());
            let attributes = self
                .attributes
                .iter()
                .filter(|a| !self.read_attributes.borrow().contains(&a.name.local_name))
                .collect::<Vec<_>>();
            if !attributes.is_empty() {
                dbg!(self.path().to_string());
                dbg!(attributes);
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("XML parsing failed: {0}")]
    XmlError(#[from] xml::reader::Error),
    #[error("Missing element at `{path}`.`{field}` of type `{ty}`")]
    ElementMissing {
        path: String,
        field: String,
        ty: String,
        backtrace: Box<Backtrace>,
    },
    #[error("A child element in `{0} is missing")]
    ChildElementIsMissing(String, Box<Backtrace>),
    #[error("Failed to parse `{path}`.`{field}` as `{ty}`: {error}")]
    ParseError {
        path: String,
        field: String,
        ty: String,
        error: ParseError,
        bt: Box<Backtrace>,
    },
    #[error("Missing attribute at `{path}`.`{field}` of type `{ty}`")]
    MissingAttribute {
        path: String,
        field: String,
        ty: String,
    },
    #[error("Invalid value for `{name}`: {value}")]
    InvalidValueFor { name: String, value: String },
}

impl Error {
    #[inline]
    pub fn missing_attribute(
        path: impl Into<String>,
        field: impl Into<String>,
        ty: impl Into<String>,
    ) -> Self {
        Self::MissingAttribute {
            path: path.into(),
            field: field.into(),
            ty: ty.into(),
        }
    }

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
    pub fn missing_element(
        path: impl Into<String>,
        field: impl Into<String>,
        ty: impl Into<String>,
    ) -> Self {
        Self::ElementMissing {
            path: path.into(),
            field: field.into(),
            ty: ty.into(),
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
    InvalidEnumValue(InvalidEnumValue),
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
        format!("{self:.17e}")
    }
}

#[derive(Debug, thiserror::Error)]
pub struct InvalidEnumValue {
    pub r#type: String,
    pub value: String,
}

impl Display for InvalidEnumValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid value for enum variant {}: {}",
            self.r#type, self.value
        )
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
                    .map_err(|e| Box::new($crate::parser::Error::from(($name, stringify!($ty), e))))
            })
            .transpose()
    };
    ($attrs:ident, $name:literal, $ty:ty) => {
        find_map_parse_attr!($attrs, $name, Option<$ty>).and_then(|v| {
            v.ok_or_else(|| {
                Box::new($crate::parser::Error::missing_attribute(
                    "<unknown>",
                    $name,
                    stringify!($ty),
                ))
            })
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
            match event.map_err($crate::parser::Error::from).map_err(Box::new)? {
                xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace: _,
                } => {
                    let mut __index = 1;
                    $(
                        if name.local_name == $name {
                            let v: $crate::parser::Result<()> = $body(attributes);
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
                        match event.map_err($crate::parser::Error::from).map_err(Box::new)? {
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
                        let v: $crate::parser::Result<()> = $alt(_event);
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
                    return Err(Box::new($crate::parser::Error::missing_element(
                        "<unknown>",
                        $name,
                        stringify!($body)
                    )));
                )?
            }
            __index += 1;
        )*
    }
}

#[macro_export]
macro_rules! match_child_eq_ignore_ascii_case {
    ($context:ident, $($name:literal $($req:literal)? => $ty:ty => $consumer:expr,)* $(_ => $alt:expr)? $(,)?) => {
        $(
            $(
                paste::paste!{
                    #[allow(non_snake_case)]
                    let mut [<__is_missing_ $name>]: bool = $req;
                }
            )?
        )*

        $context.children(|name, context| {
            match name {
                $(
                    _ if $name.eq_ignore_ascii_case(name) => {
                        let v = <$ty as TryFrom<_>>::try_from(context)?;
                        let mut c = $consumer;
                        let _ = c(v);
                        $(
                            paste::paste!{
                                let _: bool = $req;
                                [<__is_missing_ $name>] = false;
                            }
                        )?
                        Ok(())
                    },
                )*
                _ => {
                    let v: $crate::parser::Result<()> = Ok(());
                    $(
                        let mut a = $alt;
                        let v = v.and_then(|_| a(name, context));
                    )?
                    v
                }
            }
        })?;

        $(
            $(
                paste::paste!{
                    if [<__is_missing_ $name>] {
                        let _: bool = $req;
                        return Err(Box::new($crate::parser::Error::missing_element(
                            $context.path().to_string(),
                            $name,
                            stringify!($ty),
                        )));
                    };
                }
            )?
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
            type Err = $crate::parser::InvalidEnumValue;

            #[allow(deprecated)]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(_ if s.eq_ignore_ascii_case(Self::$value.as_str()) => Ok(Self::$value),)*
                    _ => Err($crate::parser::InvalidEnumValue {
                        r#type: stringify!(Self).to_string(),
                        value: s.to_string(),
                    }),
                }
            }
        }
    };
}
