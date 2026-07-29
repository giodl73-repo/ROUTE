//! Helper `indot_trafficwise_full_closure`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn indot_trafficwise_full_closure(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    text.contains("road closed")
        || text.contains("ramp closed")
        || text.contains("entrance ramp closed")
        || text.contains("exit ramp closed")
        || text.contains("freeway closed")
}

