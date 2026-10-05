//! Helper `iowa511_is_t1_relevant`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn iowa511_is_t1_relevant(route: &str, text: &str) -> bool {
    let route_norm = route.to_ascii_uppercase().replace(' ', "");
    let text_norm = text.to_ascii_uppercase();
    (route_norm.contains("I-35")
        || route_norm.contains("I35")
        || route_norm.contains("I-80")
        || route_norm.contains("I80"))
        && ["CLOSED", "CLOSURE", "CONSTRUCTION", "CRASH", "INCIDENT"]
            .iter()
            .any(|needle| text_norm.contains(needle))
}
