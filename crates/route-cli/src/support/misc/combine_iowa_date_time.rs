//! Helper `combine_iowa_date_time`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn combine_iowa_date_time(issue_date: &str, time: &str) -> String {
    if issue_date.len() != 8 || time.trim().is_empty() {
        return time.to_string();
    }
    format!(
        "{}-{}-{} {}",
        &issue_date[0..4],
        &issue_date[4..6],
        &issue_date[6..8],
        time.trim()
    )
}

