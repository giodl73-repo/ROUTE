//! Helper `tdot_smartway_event_type`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tdot_smartway_event_type(text: &str) -> &'static str {
    let text = text.to_ascii_lowercase();
    if text.contains("construction") || text.contains("maintenance") {
        "work_zone"
    } else if text.contains("crash") {
        "crash"
    } else if text.contains("closure") || text.contains("closed") {
        "closure"
    } else {
        "incident"
    }
}

