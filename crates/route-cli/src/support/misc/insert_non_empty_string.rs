//! Helper `insert_non_empty_string`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn insert_non_empty_string(target: &mut std::collections::BTreeSet<String>, value: &str) {
    if !value.trim().is_empty() {
        target.insert(value.trim().to_string());
    }
}

