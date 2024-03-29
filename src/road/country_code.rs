#[derive(Debug, Clone, PartialEq)]
pub enum CountryCode {
    CountryCodeDeprecated(CountryCodeDeprecated),
    Iso3166alpha2(String),
    #[deprecated]
    Iso3166alpha3(String),
}

impl CountryCode {
    #[allow(deprecated)]
    pub fn as_str(&self) -> &str {
        match self {
            Self::CountryCodeDeprecated(code) => code.as_str(),
            Self::Iso3166alpha2(v) => v,
            Self::Iso3166alpha3(v) => v,
        }
    }
}

impl core::str::FromStr for CountryCode {
    type Err = crate::parser::InvalidEnumValue;

    #[allow(deprecated)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 2 && s.chars().all(|c| c.is_ascii_alphabetic()) {
            Ok(Self::Iso3166alpha2(s.to_string()))
        } else if s.len() == 3 && s.chars().all(|c| c.is_ascii_alphabetic()) {
            Ok(Self::Iso3166alpha3(s.to_string()))
        } else if let Ok(cc) = CountryCodeDeprecated::from_str(s) {
            Ok(Self::CountryCodeDeprecated(cc))
        } else {
            Err(crate::parser::InvalidEnumValue {
                r#type: core::any::type_name::<Self>().to_string(),
                value: s.to_string(),
            })
        }
    }
}

#[cfg(feature = "fuzzing")]
impl arbitrary::Arbitrary<'_> for CountryCode {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        use crate::fuzzing::ArbitraryStrings;
        if u.arbitrary()? {
            Ok(Self::CountryCodeDeprecated(u.arbitrary()?))
        } else if u.arbitrary()? {
            Ok(Self::Iso3166alpha2(
                u.arbitrary_string(2..=2, &['A'..='Z'])?,
            ))
        } else {
            #[allow(deprecated)]
            Ok(Self::Iso3166alpha3(
                u.arbitrary_string(3..=3, &['A'..='Z'])?,
            ))
        }
    }
}

#[deprecated]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum CountryCodeDeprecated {
    OpenDRIVE,
    Austria,
    Brazil,
    China,
    France,
    Germany,
    Italy,
    Switzerland,
    // USA, This has been removed, because it overlaps with Iso3166alpha3("USA")
}

impl_from_str_as_str!(
    CountryCodeDeprecated,
    "OpenDRIVE" => OpenDRIVE,
    "Austria" => Austria,
    "Brazil" => Brazil,
    "China" => China,
    "France" => France,
    "Germany" => Germany,
    "Italy" => Italy,
    "Switzerland" => Switzerland,
    // "USA" => USA, This has been removed, because it overlaps with Iso3166alpha3("USA")
);
