//! Helper `join_string_set`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn join_string_set(values: &std::collections::BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join(";")
}
