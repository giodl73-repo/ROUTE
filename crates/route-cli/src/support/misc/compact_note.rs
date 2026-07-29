//! Helper `compact_note`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn compact_note(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

