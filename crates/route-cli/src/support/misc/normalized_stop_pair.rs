//! Helper `normalized_stop_pair`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn normalized_stop_pair(a: &str, b: &str) -> String {
    if a <= b {
        format!("{a}->{b}")
    } else {
        format!("{b}->{a}")
    }
}

