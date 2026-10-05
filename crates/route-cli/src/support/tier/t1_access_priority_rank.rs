//! Helper `t1_access_priority_rank`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_access_priority_rank(priority: &str) -> u8 {
    match priority {
        "critical" => 0,
        "high" => 1,
        "medium" => 2,
        _ => 3,
    }
}
