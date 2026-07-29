//! Helper `t2_transfer_complexity_band`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_transfer_complexity_band(transfer_stop_count: usize) -> &'static str {
    if transfer_stop_count >= 7 {
        "severe-transfer-complexity"
    } else if transfer_stop_count >= 6 {
        "high-transfer-complexity"
    } else {
        "moderate-transfer-complexity"
    }
}

