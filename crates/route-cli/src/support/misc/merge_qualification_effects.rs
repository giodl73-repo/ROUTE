//! Helper `merge_qualification_effects`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn merge_qualification_effects(left: &str, right: &str) -> String {
    let mut values = std::collections::BTreeSet::new();
    for value in left.split('|').chain(right.split('|')).map(str::trim) {
        if !value.is_empty() {
            values.insert(value.to_string());
        }
    }
    join_pipe_set(&values)
}

