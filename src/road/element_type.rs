#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum ElementType {
    Road,
    Junction,
}

impl_from_str_as_str!(
    ElementType,
    "road" => Road,
    "junction" => Junction,
);
