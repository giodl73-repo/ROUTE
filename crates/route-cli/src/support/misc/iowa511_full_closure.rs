//! Helper `iowa511_full_closure`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn iowa511_full_closure(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    if text.contains("shoulder") || text.contains("lane closed") || text.contains("lanes closed") {
        return false;
    }
    text.contains("road closed")
        || text.contains("ramp closed")
        || text.contains("entrance ramp closed")
        || text.contains(": closed")
}

