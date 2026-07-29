//! Helper `stop_candidate_selector_score`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_candidate_selector_score(row: &StopCandidateRow) -> u16 {
    stop_class_selector_score(&row.requested_class) * 100
        + selector_signal_score(&row.transfer_value) * 12
        + selector_signal_score(&row.freight_volume) * 12
        + selector_signal_score(&row.resilience_value) * 8
        + selector_signal_score(&row.land_ops_feasibility) * 4
        + selector_signal_score(&row.equity_community)
}

