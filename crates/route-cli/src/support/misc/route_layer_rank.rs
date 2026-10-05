//! Helper `route_layer_rank`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn route_layer_rank(column_decision: &str) -> u8 {
    match column_decision {
        "selected" => 0,
        "upward-review" => 1,
        "review" => 2,
        _ => 3,
    }
}
