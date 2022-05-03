#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum ElementDir {
    Plus,
    Minus,
}

impl_from_str_as_str!(
    ElementDir,
    "+" => Plus,
    "-" => Minus,
);
