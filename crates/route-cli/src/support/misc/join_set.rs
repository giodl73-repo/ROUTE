//! Helper `join_set`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn join_set(values: &std::collections::BTreeSet<&str>) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.iter().copied().collect::<Vec<_>>().join(", ")
    }
}

