//! Helper `mdot_midrive_full_closure`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn mdot_midrive_full_closure(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    text.contains("all lanes")
        || text.contains("road closed")
        || text.contains("freeway closed")
        || text.contains("ramp closed")
}

