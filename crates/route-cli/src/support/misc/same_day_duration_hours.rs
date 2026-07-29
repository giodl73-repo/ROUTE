//! Helper `same_day_duration_hours`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn same_day_duration_hours(start: &str, end: &str) -> Option<f64> {
    let start = parse_12h_minutes(start)?;
    let end = parse_12h_minutes(end)?;
    if end >= start {
        Some((end - start) as f64 / 60.0)
    } else {
        None
    }
}

