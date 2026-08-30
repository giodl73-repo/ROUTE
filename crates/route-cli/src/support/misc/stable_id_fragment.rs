//! Helper `stable_id_fragment`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stable_id_fragment(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_uppercase()
}
