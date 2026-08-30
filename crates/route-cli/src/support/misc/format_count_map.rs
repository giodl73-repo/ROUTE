//! Helper `format_count_map`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn format_count_map(counts: &std::collections::BTreeMap<String, usize>) -> String {
    if counts.is_empty() {
        "none".to_string()
    } else {
        counts
            .iter()
            .map(|(key, count)| format!("{key}: {count}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}
