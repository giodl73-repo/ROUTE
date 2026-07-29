//! Helper `insert_semicolon_values`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn insert_semicolon_values(target: &mut std::collections::BTreeSet<String>, value: &str) {
    for item in value
        .split(';')
        .map(str::trim)
        .filter(|item| !item.is_empty())
    {
        target.insert(item.to_string());
    }
}

