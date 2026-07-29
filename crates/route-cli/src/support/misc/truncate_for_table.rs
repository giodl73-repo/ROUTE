//! Helper `truncate_for_table`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn truncate_for_table(value: &str, width: usize) -> String {
    if value.chars().count() <= width {
        value.to_string()
    } else {
        value
            .chars()
            .take(width.saturating_sub(1))
            .collect::<String>()
            + "…"
    }
}

