use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum ConnectionType {
    Default,
    Vertical,
}

#[derive(Debug, Clone)]
pub enum ContactPoint {
    Start,
    End,
}

impl FromStr for ContactPoint {
    type Err = crate::parser::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("start") => Ok(Self::Start),
            _ if s.eq_ignore_ascii_case("end") => Ok(Self::End),
            _ => Err(crate::parser::Error::invalid_value_for::<Self, _>(s)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElementDir {
    Plus,
    Minus,
}

impl FromStr for ElementDir {
    type Err = crate::parser::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            _ => Err(crate::parser::Error::invalid_value_for::<Self, _>(s)),
        }
    }
}
