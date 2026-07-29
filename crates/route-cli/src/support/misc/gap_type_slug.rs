//! Helper `gap_type_slug`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn gap_type_slug(gap_type: &GapType) -> &'static str {
    match gap_type {
        GapType::MissingLink => "missing-link",
        GapType::Bottleneck => "bottleneck",
        GapType::Resilience => "resilience",
        GapType::Intermodal => "intermodal",
    }
}

