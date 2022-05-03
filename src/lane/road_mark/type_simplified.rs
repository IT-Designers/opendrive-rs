/// The known keywords for the simplified road mark type information
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum TypeSimplified {
    None,
    Solid,
    Broken,
    /// for double solid line
    SolidSolid,
    /// from inside to outside, exception: center lane – from left to right
    SolidBroken,
    /// from inside to outside, exception: center lane – from left to right
    BrokenSolid,
    /// from inside to outside, exception: center lane – from left to right
    BrokenBroken,
    BottsDots,
    /// meaning a grass edge
    Grass,
    Curb,
    /// if detailed description is given in child tags (via [`Type`])
    Custom,
    /// describing the limit of usable space on a road
    Edge,
}

impl_from_str_as_str!(
    TypeSimplified,
    "none" => None,
    "solid" => Solid,
    "broken" => Broken,
    "solid solid" => SolidSolid,
    "solid broken" => SolidBroken,
    "broken solid" => BrokenSolid,
    "broken broken" => BrokenBroken,
    "botts dots" => BottsDots,
    "grass" => Grass,
    "curb" => Curb,
    "custom" => Custom,
    "edge" => Edge,
);
