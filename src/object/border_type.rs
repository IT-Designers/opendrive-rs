#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum BorderType {
    Concrete,
    Curb,
}

impl_from_str_as_str!(
    BorderType,
    "concrete" => Concrete,
    "curb" => Curb,
);
