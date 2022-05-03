use crate::lane::border::Border;
use crate::lane::width::Width;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum LaneChoice {
    Border(Border),
    Width(Width),
}
