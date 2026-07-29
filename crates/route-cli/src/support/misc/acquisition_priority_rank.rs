//! Helper `acquisition_priority_rank`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn acquisition_priority_rank(priority: &str) -> u8 {
    match priority {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => 3,
    }
}

