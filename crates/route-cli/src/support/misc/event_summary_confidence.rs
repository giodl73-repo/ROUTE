//! Helper `event_summary_confidence`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn event_summary_confidence(rows: &[&T1FailureEventRow]) -> String {
    if rows.is_empty() {
        return "unknown".to_string();
    }
    if rows
        .iter()
        .all(|row| row.confidence.eq_ignore_ascii_case("high"))
    {
        "high".to_string()
    } else if rows
        .iter()
        .any(|row| row.confidence.eq_ignore_ascii_case("low"))
    {
        "low".to_string()
    } else {
        "medium".to_string()
    }
}

