//! Helper `mdot_midrive_is_t1_relevant`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn mdot_midrive_is_t1_relevant(text: &str) -> bool {
    let text_norm = text.to_ascii_uppercase().replace(' ', "");
    (text_norm.contains("I-75")
        || text_norm.contains("I75")
        || text_norm.contains("I-94")
        || text_norm.contains("I94")
        || text_norm.contains("I-96")
        || text_norm.contains("I96")
        || text_norm.contains("I-275")
        || text_norm.contains("I275")
        || text_norm.contains("I-696")
        || text_norm.contains("I696"))
        && ["CLOSURE", "CLOSED", "CRASH", "INCIDENT", "CONSTRUCTION"]
            .iter()
            .any(|needle| text.to_ascii_uppercase().contains(needle))
}

