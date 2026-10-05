//! Helper `board_layer_rank`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn board_layer_rank(board_layer: &str) -> u8 {
    match board_layer {
        "zone-summary" => 0,
        "selected-route" => 1,
        "review-connector" => 2,
        "held-gap" => 3,
        "unassigned-gap-backlog" => 4,
        _ => 5,
    }
}
