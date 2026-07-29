//! Helper `join_limited_set`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn join_limited_set(values: &std::collections::BTreeSet<String>, limit: usize) -> String {
    values
        .iter()
        .take(limit)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(";")
}

