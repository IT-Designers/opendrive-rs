use inertial::PositionInertial;
use road::PositionRoad;

pub mod inertial;
pub mod road;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Position {
    Inertial(PositionInertial),
    Road(PositionRoad),
}
