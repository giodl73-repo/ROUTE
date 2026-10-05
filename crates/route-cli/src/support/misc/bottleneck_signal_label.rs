//! Helper `bottleneck_signal_label`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn bottleneck_signal_label(row: &ScoreSignalRow) -> &'static str {
    if row.a1 >= 7.0 || row.a3 >= 7.0 {
        "corridor_stress"
    } else if row.b2 >= 8.0 {
        "topology_chokepoint"
    } else {
        "capacity_needs_flow"
    }
}
