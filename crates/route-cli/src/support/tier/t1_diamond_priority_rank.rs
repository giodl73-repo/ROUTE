//! Helper `t1_diamond_priority_rank`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_diamond_priority_rank(priority: &str) -> usize {
    match priority.trim().to_ascii_uppercase().as_str() {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => 99,
    }
}
