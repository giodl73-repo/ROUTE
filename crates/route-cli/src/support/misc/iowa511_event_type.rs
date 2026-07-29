//! Helper `iowa511_event_type`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn iowa511_event_type(text: &str) -> &'static str {
    let text = text.to_ascii_lowercase();
    if text.contains("construction") {
        "work_zone"
    } else if text.contains("crash") {
        "crash"
    } else if text.contains("closed") || text.contains("closure") {
        "closure"
    } else {
        "incident"
    }
}

