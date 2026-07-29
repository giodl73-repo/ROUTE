//! Helper `indot_trafficwise_is_t1_relevant`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn indot_trafficwise_is_t1_relevant(text: &str) -> bool {
    let text_norm = text.to_ascii_uppercase().replace(' ', "");
    (text_norm.contains("I-80")
        || text_norm.contains("I80")
        || text_norm.contains("I-90")
        || text_norm.contains("I90")
        || text_norm.contains("I-94")
        || text_norm.contains("I94")
        || text_norm.contains("TOLLROAD"))
        && [
            "CLOSURE",
            "CLOSED",
            "CRASH",
            "INCIDENT",
            "CONSTRUCTION",
            "ROADWORK",
            "LANE CLOSED",
        ]
        .iter()
        .any(|needle| text.to_ascii_uppercase().contains(needle))
}

