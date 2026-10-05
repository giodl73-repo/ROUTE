//! Helper `tdot_smartway_is_t1_relevant`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tdot_smartway_is_t1_relevant(road_names: &str, text: &str) -> bool {
    let route_norm = road_names.to_ascii_uppercase().replace(' ', "");
    let text_norm = text.to_ascii_uppercase().replace(' ', "");
    (route_norm.contains("I-40")
        || route_norm.contains("I40")
        || route_norm.contains("I-75")
        || route_norm.contains("I75")
        || text_norm.contains("I-40")
        || text_norm.contains("I40")
        || text_norm.contains("I-75")
        || text_norm.contains("I75"))
        && ["CLOSURE", "CLOSED", "CRASH", "INCIDENT", "CONSTRUCTION"]
            .iter()
            .any(|needle| text.to_ascii_uppercase().contains(needle))
}
