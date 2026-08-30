//! Helper `looks_like_commit_ref`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn looks_like_commit_ref(value: &str) -> bool {
    (7..=40).contains(&value.len()) && value.chars().all(|ch| ch.is_ascii_hexdigit())
}
