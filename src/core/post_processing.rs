#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum PostProcessing {
    Raw,
    Cleaned,
    Processed,
    Fused,
}

impl_from_str_as_str!(
    PostProcessing,
    "raw" => Raw,
    "cleaned" => Cleaned,
    "processed" => Processed,
    "fused" => Fused,
);
