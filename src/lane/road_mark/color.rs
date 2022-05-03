/// The known keywords for the road mark color information
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Color {
    /// equivalent to [`Color::White`]
    Standard,
    Blue,
    Green,
    Red,
    White,
    Yellow,
    Orange,
    Violet,
}

impl_from_str_as_str!(
    Color,
    "standard" => Standard,
    "blue" => Blue,
    "green" => Green,
    "red" => Red,
    "white" => White,
    "yellow" => Yellow,
    "orange" => Orange,
    "violet" => Violet,
);
