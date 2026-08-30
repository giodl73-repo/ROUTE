//! Helper `pavement_acquisition_action`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pavement_acquisition_action(
    route_count: usize,
    blocked_member_count: usize,
) -> (&'static str, &'static str) {
    if route_count >= 3 || blocked_member_count >= 80 {
        (
            "A",
            "refresh HPMS/state pavement feed for broad multi-route coverage",
        )
    } else if route_count == 2 || blocked_member_count >= 30 {
        (
            "B",
            "refresh HPMS/state pavement feed for targeted corridor coverage",
        )
    } else {
        (
            "C",
            "fill targeted pavement rows from HPMS or state DOT asset feed",
        )
    }
}
